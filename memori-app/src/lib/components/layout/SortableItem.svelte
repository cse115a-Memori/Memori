<script lang="ts">
	import type { Data, Type, UniqueIdentifier } from '@dnd-kit/abstract'
	import { useSortable } from '@dnd-kit-svelte/svelte/sortable'
	import { EllipsisVertical } from '@lucide/svelte'
	import { onMount } from 'svelte'
	import type { ClassValue } from 'svelte/elements'
	import { Button } from '@/components/ui/button'
	import * as Drawer from '@/components/ui/drawer'
	import { Input } from '@/components/ui/input'
	import * as Select from '@/components/ui/select'
	import { githubState } from '@/features/github'
	import { prefsState } from '@/features/prefs/store'
	import { kindToDisplay, type WidgetView } from '@/features/widgets/model/widget-frame'
	import { getWidgetKinds } from '@/features/widgets/service'
	import { syncWidgets, updateWidgetKind } from '@/features/widgets/widgets-store'
	import { commands, tryCmd } from '@/tauri'
	import { cn } from '@/utils'
	import { cardCls } from './sortable-item-classes'
	import {
		buildKindFromDraft,
		createDraftFromKind,
		isDraftPersistable,
		isKindEditable,
		kindSignature,
		type SortableItemDraft,
	} from './sortable-item-domain'
	import {
		CLOCK_TIMEZONE_OPTIONS,
		formatCompactClock,
		getClockTimezoneLabel,
		getCurrentSystemTimeZone,
		resolveClockTimeZone,
		toClockTimezoneDraftValue,
	} from './widget-clock'

	interface Props {
		id: UniqueIdentifier
		index: number
		group?: UniqueIdentifier
		type?: Type
		data: Data
		widget: WidgetView
		cls: ClassValue
		isOverlay?: boolean
	}

	let {
		id,
		index,
		group,
		type,
		data,
		widget = $bindable(),
		cls,
		isOverlay = false,
	}: Props = $props()

	const display = $derived(kindToDisplay(widget.kind))
	const isClock = $derived('Clock' in widget.kind)
	let now = $state(new Date())
	let isEditorOpen = $state(false)
	const systemTimeZone = getCurrentSystemTimeZone()
	let editorState: {
		draft: SortableItemDraft
		sourceKindSignature: string
	} = $state({
		draft: createDraftFromKind(widget.kind),
		sourceKindSignature: kindSignature(widget.kind),
	})
	const isEditable = $derived(isKindEditable(widget.kind))
	const kindSignatureNow = $derived(kindSignature(widget.kind))

	const compactClock = $derived(
		formatCompactClock(now, prefsState.systemOptions.timeZone ?? undefined)
	)
	const clockTimezoneOptions = $derived(
		(() => {
			const options = systemTimeZone
				? [
						systemTimeZone,
						...CLOCK_TIMEZONE_OPTIONS.filter(option => option !== systemTimeZone),
					]
				: [...CLOCK_TIMEZONE_OPTIONS]
			const hasCurrentSelection = options.includes(editorState.draft.clockTimeZone)

			if (
				hasCurrentSelection ||
				editorState.draft.clockTimeZone === '' ||
				editorState.draft.clockTimeZone === undefined
			) {
				return options
			}

			return [editorState.draft.clockTimeZone, ...options]
		})()
	)
	const clockTimezoneLabel = $derived(
		getClockTimezoneLabel(editorState.draft.clockTimeZone)
	)
	const clockPreview = $derived(
		formatCompactClock(now, resolveClockTimeZone(editorState.draft.clockTimeZone))
	)

	const { ref, isDragging } = useSortable({
		id: () => id,
		index: () => index,
		group: () => group,
		type: () => type,
		data: () => data,
		transition: {
			idle: true,
		},
		feedback: 'move' as const,
	})

	onMount(() => {
		if (!('Clock' in widget.kind)) return

		const intervalId = setInterval(() => {
			now = new Date()
		}, 1000)

		return () => clearInterval(intervalId)
	})

	function loadDraftFromKind(kind: WidgetView['kind']): void {
		const nextDraft = createDraftFromKind(kind)
		if ('Clock' in kind) {
			nextDraft.clockTimeZone = toClockTimezoneDraftValue(
				prefsState.systemOptions.timeZone,
				systemTimeZone
			)
		}
		editorState.draft = nextDraft
		editorState.sourceKindSignature = kindSignature(kind)
	}

	function openEditor(): void {
		const currentKindSignature = kindSignature(widget.kind)
		if (
			'Clock' in widget.kind ||
			editorState.sourceKindSignature !== currentKindSignature
		) {
			loadDraftFromKind(widget.kind)
		}
		setEditorOpen(true)
		if ('Github' in widget.kind) {
			void loadRepos()
		}
	}

	const canSave = $derived(
		isEditable &&
			isDraftPersistable(widget.kind, editorState.draft) &&
			kindSignatureNow === editorState.sourceKindSignature
	)

	function applyDraftChanges(): void {
		if (!canSave) return

		if ('Clock' in widget.kind) {
			prefsState.systemOptions.timeZone =
				resolveClockTimeZone(editorState.draft.clockTimeZone) ?? null
			loadDraftFromKind(widget.kind)
			return
		}

		const nextKind = buildKindFromDraft(widget.kind, editorState.draft)
		if (!nextKind) return

		updateWidgetKind(widget.widgetId, nextKind)
		widget = { ...widget, kind: nextKind }
		loadDraftFromKind(nextKind)

		if ('Github' in nextKind && editorState.draft.githubRepo) {
			githubState.repo = editorState.draft.githubRepo
			void getWidgetKinds().then(result => {
				result.match(
					widgets => syncWidgets(widgets),
					error => console.error('Failed to refresh widgets:', error)
				)
			})
		}
	}

	function handleSave(event: SubmitEvent): void {
		event.preventDefault()
		if (!canSave) {
			return
		}
		setEditorOpen(false)
	}

	function setEditorOpen(nextOpen: boolean): void {
		if (isEditorOpen && !nextOpen) {
			applyDraftChanges()
		}

		isEditorOpen = nextOpen
	}

	let repos = $state<string[]>([])
	let reposLoading = $state(false)

	async function loadRepos(): Promise<void> {
		reposLoading = true
		await tryCmd(commands.getGithubRepos()).match(
			result => {
				repos = result
			},
			error => {
				console.error('Failed to load repos:', error)
			}
		)
		reposLoading = false
	}
</script>

<Drawer.Root bind:open={() => isEditorOpen, setEditorOpen}>
	<div class={cn('relative select-none', cls)} {@attach ref}>
		<div
			class={[
				cardCls.Base,
				cardCls.Interactive,
				{ 'cursor-grabbing': isDragging.current || isOverlay },
				{ invisible: isDragging.current && !isOverlay },
				'w-full h-full',
			]}
		>
			<section class="flex justify-between items-center mb-2">
				<div class={cardCls.Title}>{display.name}</div>
				<Drawer.Trigger class="cursor-pointer" onclick={openEditor}>
					<EllipsisVertical size={16} />
				</Drawer.Trigger>
			</section>
			{#if isClock}
				<p class="text-sm font-semibold tabular-nums text-slate-700">
					{compactClock.time}
				</p>
				<p class="text-xs text-slate-500">{compactClock.zone}</p>
			{:else}
				<p class={cardCls.Content}>{display.content}</p>
			{/if}
		</div>

		{#if !isOverlay && isDragging.current}
			<div class="absolute inset-0 flex items-center justify-center h-full w-full">
				<div class={cardCls.Placeholder}>
					<span class={cardCls.PlaceholderText}>Moving: {display.name}</span>
				</div>
			</div>
		{/if}
	</div>

	<Drawer.Content>
		<form class="mx-auto w-full max-w-sm p-4 pt-3 space-y-4" onsubmit={handleSave}>
			<Drawer.Header>
				<Drawer.Title>Editing {display.name}</Drawer.Title>
				<Drawer.Description>
					Changes are applied automatically when you close this editor.
				</Drawer.Description>
			</Drawer.Header>

			{#if 'Name' in widget.kind}
				<label class="space-y-1 block">
					<span class="text-sm font-medium text-slate-700">Name</span>
					<Input bind:value={editorState.draft.name} placeholder="Display name" />
				</label>
			{:else if 'Clock' in widget.kind}
				<div class="space-y-3">
					<div class="space-y-1">
						<span class="text-sm font-medium text-slate-700">Timezone</span>
						<Select.Root
							type="single"
							name="clockTimezone"
							value={editorState.draft.clockTimeZone}
							onValueChange={value => {
								editorState.draft.clockTimeZone = value
							}}
						>
							<Select.Trigger class="w-full"> {clockTimezoneLabel} </Select.Trigger>
							<Select.Content>
								<Select.Group>
									<Select.Label>Timezones</Select.Label>
									{#each clockTimezoneOptions as option (option)}
										<Select.Item value={option} label={getClockTimezoneLabel(option)}>
											{getClockTimezoneLabel(option)}
										</Select.Item>
									{/each}
								</Select.Group>
							</Select.Content>
						</Select.Root>
					</div>
					<p class="text-xs text-slate-500">
						Preview: {clockPreview.time} {clockPreview.zone}
					</p>
				</div>
			{:else if 'Weather' in widget.kind}
				<div class="space-y-3">
					<label class="space-y-1 block">
						<span class="text-sm font-medium text-slate-700">Stop</span>
						<Input
							bind:value={editorState.draft.weatherCity}
							placeholder="Santa Cruz"
						/>
					</label>
				</div>
			{:else if 'Bus' in widget.kind}
				<div class="space-y-3">
					<label class="space-y-1 block">
						<span class="text-sm font-medium text-slate-700">Stop</span>
						<Input bind:value={editorState.draft.busStop} placeholder="1230" />
					</label>
				</div>
			{:else if 'Twitch' in widget.kind}
				<label class="space-y-1 block">
					<span class="text-sm font-medium text-slate-700">User</span>
					<Input bind:value={editorState.draft.twitchUser} placeholder="streamer" />
				</label>
			{:else if 'Github' in widget.kind}
				<div class="space-y-1">
					<span class="text-sm font-medium text-slate-700">Repository</span>
					{#if reposLoading}
						<p class="text-sm text-slate-500">Loading repos...</p>
					{:else}
						<div class="flex max-h-64 flex-col gap-2 overflow-y-auto">
							{#each repos as repo (repo)}
								{@const owner = repo.split('/')[0]}
								{@const repoName = repo.split('/')[1]}
								<button
									type="button"
									onclick={() => {
										editorState.draft.githubRepo = repo
									}}
									class={[
										'flex items-center gap-3 rounded-lg border p-2 text-left transition-colors',
										editorState.draft.githubRepo === repo
											? 'border-sky-400 bg-sky-50'
											: 'border-slate-200 hover:bg-slate-50',
									]}
								>
									<img
										src={`https://github.com/${owner}.png?size=32`}
										alt={owner}
										class="h-8 w-8 rounded-full"
									/>
									<div class="flex flex-col">
										<span class="text-sm font-medium text-slate-700">{repoName}</span>
										<span class="text-xs text-slate-400">{owner}</span>
									</div>
								</button>
							{/each}
						</div>
					{/if}
				</div>
			{:else}
				<p class="text-sm text-slate-500">
					{display.name}
					is currently read-only in this editor.
				</p>
			{/if}

			<div class="flex justify-end gap-2 pt-1">
				<Button
					type="button"
					variant="ghost"
					onclick={() => loadDraftFromKind(widget.kind)}
				>
					Reset
				</Button>
			</div>
		</form>
	</Drawer.Content>
</Drawer.Root>
