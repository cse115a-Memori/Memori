<script lang="ts">
	import type { Data, Type, UniqueIdentifier } from '@dnd-kit/abstract'
	import { useSortable } from '@dnd-kit-svelte/svelte/sortable'
	import { EllipsisVertical } from '@lucide/svelte'
	import { onMount } from 'svelte'
	import type { ClassValue } from 'svelte/elements'
	import { Button } from '@/components/ui/button'
	import * as Drawer from '@/components/ui/drawer'
	import { Input } from '@/components/ui/input'
	import { prefsState } from '@/features/prefs/store.ts'
	import {
		kindToDisplay,
		type WidgetView,
	} from '@/features/widgets/model/widget-frame.ts'
	import { updateWidgetKind } from '@/features/widgets/widgets-store.ts'
	import { cn } from '@/utils.ts'
	import {
		sortableCardBaseClasses,
		sortableCardContentClasses,
		sortableCardInteractiveClasses,
		sortableCardPlaceholderClasses,
		sortableCardPlaceholderTextClasses,
		sortableCardTitleClasses,
	} from './sortable-item-classes.ts'
	import { formatCompactClock } from './widget-clock.ts'

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
	let hasDraft = $state(false)
	let draftSourceKindSignature = $state<string | null>(null)

	let nameDraft = $state('')
	let clockHoursDraft = $state<number | undefined>(undefined)
	let clockMinutesDraft = $state<number | undefined>(undefined)
	let clockSecondsDraft = $state<number | undefined>(undefined)
	let weatherTempDraft = $state('')
	let weatherIconDraft = $state('')
	let busRouteDraft = $state('')
	let busPredictionDraft = $state('')
	let twitchUserDraft = $state('')
	let unsupportedWidgetKind = $state('Unknown')

	const compactClock = $derived(
		formatCompactClock(now, prefsState.systemOptions.timeZone ?? undefined)
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

	function clampUnit(value: number | undefined, max: number, fallback: number): number {
		if (value === undefined || Number.isNaN(value)) return fallback
		const truncated = Math.trunc(value)
		return Math.min(max, Math.max(0, truncated))
	}

	function getKindSignature(kind: WidgetView['kind']): string {
		return JSON.stringify(kind)
	}

	function loadDraftFromKind(kind: WidgetView['kind']): void {
		if ('Name' in kind) {
			nameDraft = kind.Name.name
			hasDraft = true
			draftSourceKindSignature = getKindSignature(kind)
			return
		}

		if ('Clock' in kind) {
			clockHoursDraft = kind.Clock.hours
			clockMinutesDraft = kind.Clock.minutes
			clockSecondsDraft = kind.Clock.seconds
			hasDraft = true
			draftSourceKindSignature = getKindSignature(kind)
			return
		}

		if ('Weather' in kind) {
			weatherTempDraft = kind.Weather.temp
			weatherIconDraft = kind.Weather.icon
			hasDraft = true
			draftSourceKindSignature = getKindSignature(kind)
			return
		}

		if ('Bus' in kind) {
			busRouteDraft = kind.Bus.route
			busPredictionDraft = kind.Bus.prediction
			hasDraft = true
			draftSourceKindSignature = getKindSignature(kind)
			return
		}

		if ('Twitch' in kind) {
			twitchUserDraft = kind.Twitch.user
			hasDraft = true
			draftSourceKindSignature = getKindSignature(kind)
			return
		}

		if ('Github' in kind) {
			unsupportedWidgetKind = 'Github'
			hasDraft = true
			draftSourceKindSignature = getKindSignature(kind)
			return
		}

		unsupportedWidgetKind = 'Unknown'
		hasDraft = true
		draftSourceKindSignature = getKindSignature(kind)
	}

	function loadDraftFromWidget(): void {
		loadDraftFromKind(widget.kind)
	}

	function resetDraftFromWidget(): void {
		loadDraftFromWidget()
	}

	function openEditor(): void {
		const currentKindSignature = getKindSignature(widget.kind)
		if (!hasDraft || draftSourceKindSignature !== currentKindSignature) {
			loadDraftFromWidget()
		}
		isEditorOpen = true
	}

	function canSave(): boolean {
		if ('Name' in widget.kind) {
			return nameDraft.trim().length > 0
		}

		if ('Clock' in widget.kind) {
			return true
		}

		if ('Weather' in widget.kind) {
			return weatherTempDraft.trim().length > 0 && weatherIconDraft.trim().length > 0
		}

		if ('Bus' in widget.kind) {
			return busRouteDraft.trim().length > 0 && busPredictionDraft.trim().length > 0
		}

		if ('Twitch' in widget.kind) {
			return twitchUserDraft.trim().length > 0
		}

		return false
	}

	function handleSave(event: SubmitEvent): void {
		event.preventDefault()
		if (!canSave()) return

		let nextKind = widget.kind

		if ('Name' in widget.kind) {
			nextKind = {
				Name: {
					name: nameDraft.trim(),
				},
			}
		}

		if ('Clock' in widget.kind) {
			nextKind = {
				Clock: {
					hours: clampUnit(clockHoursDraft, 23, widget.kind.Clock.hours),
					minutes: clampUnit(clockMinutesDraft, 59, widget.kind.Clock.minutes),
					seconds: clampUnit(clockSecondsDraft, 59, widget.kind.Clock.seconds),
				},
			}
		}

		if ('Weather' in widget.kind) {
			nextKind = {
				Weather: {
					temp: weatherTempDraft.trim(),
					icon: weatherIconDraft.trim(),
				},
			}
		}

		if ('Bus' in widget.kind) {
			nextKind = {
				Bus: {
					route: busRouteDraft.trim(),
					prediction: busPredictionDraft.trim(),
				},
			}
		}

		if ('Twitch' in widget.kind) {
			nextKind = {
				Twitch: {
					user: twitchUserDraft.trim(),
				},
			}
		}

		updateWidgetKind(widget.widgetId, nextKind)
		widget = { ...widget, kind: nextKind }
		loadDraftFromKind(nextKind)
		isEditorOpen = false
	}
</script>

<Drawer.Root bind:open={isEditorOpen}>
	<div class={cn('relative select-none', cls)} {@attach ref}>
		<div
			class={[
				sortableCardBaseClasses,
				sortableCardInteractiveClasses,
				{ 'cursor-grabbing': isDragging.current || isOverlay },
				{ invisible: isDragging.current && !isOverlay },
				'w-full h-full',
			]}
		>
			<section class="flex justify-between items-center mb-2">
				<div class={sortableCardTitleClasses}>{display.name}</div>
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
				<p class={sortableCardContentClasses}>{display.content}</p>
			{/if}
		</div>

		{#if !isOverlay && isDragging.current}
			<div class="absolute inset-0 flex items-center justify-center h-full w-full">
				<div class={sortableCardPlaceholderClasses}>
					<span class={sortableCardPlaceholderTextClasses}>Moving: {display.name}</span>
				</div>
			</div>
		{/if}
	</div>

	<Drawer.Content>
		<form class="mx-auto w-full max-w-sm p-4 pt-3 space-y-4" onsubmit={handleSave}>
			<Drawer.Header>
				<Drawer.Title>Editing {display.name}</Drawer.Title>
				<Drawer.Description>
					Changes are applied only after you press Save.
				</Drawer.Description>
			</Drawer.Header>

			{#if 'Name' in widget.kind}
				<label class="space-y-1 block">
					<span class="text-sm font-medium text-slate-700">Name</span>
					<Input bind:value={nameDraft} placeholder="Display name" />
				</label>
			{:else if 'Clock' in widget.kind}
				<div class="grid grid-cols-3 gap-2">
					<label class="space-y-1 block">
						<span class="text-sm font-medium text-slate-700">Hours</span>
						<Input type="number" min="0" max="23" bind:value={clockHoursDraft} />
					</label>
					<label class="space-y-1 block">
						<span class="text-sm font-medium text-slate-700">Minutes</span>
						<Input type="number" min="0" max="59" bind:value={clockMinutesDraft} />
					</label>
					<label class="space-y-1 block">
						<span class="text-sm font-medium text-slate-700">Seconds</span>
						<Input type="number" min="0" max="59" bind:value={clockSecondsDraft} />
					</label>
				</div>
			{:else if 'Weather' in widget.kind}
				<div class="space-y-3">
					<label class="space-y-1 block">
						<span class="text-sm font-medium text-slate-700">Temp</span>
						<Input bind:value={weatherTempDraft} placeholder="24" />
					</label>
					<label class="space-y-1 block">
						<span class="text-sm font-medium text-slate-700">Icon</span>
						<Input bind:value={weatherIconDraft} placeholder="sunny" />
					</label>
				</div>
			{:else if 'Bus' in widget.kind}
				<div class="space-y-3">
					<label class="space-y-1 block">
						<span class="text-sm font-medium text-slate-700">Route</span>
						<Input bind:value={busRouteDraft} placeholder="15A" />
					</label>
					<label class="space-y-1 block">
						<span class="text-sm font-medium text-slate-700">Prediction</span>
						<Input bind:value={busPredictionDraft} placeholder="7 mins" />
					</label>
				</div>
			{:else if 'Twitch' in widget.kind}
				<label class="space-y-1 block">
					<span class="text-sm font-medium text-slate-700">User</span>
					<Input bind:value={twitchUserDraft} placeholder="streamer" />
				</label>
			{:else}
				<p class="text-sm text-slate-500">
					{unsupportedWidgetKind}
					is currently read-only in this editor.
				</p>
			{/if}

			<div class="flex justify-end gap-2 pt-1">
				<Button type="button" variant="outline" onclick={() => (isEditorOpen = false)}>
					Close
				</Button>
				<Button type="button" variant="ghost" onclick={resetDraftFromWidget}>
					Reset
				</Button>
				<Button type="submit" disabled={!canSave()}>Save</Button>
			</div>
		</form>
	</Drawer.Content>
</Drawer.Root>
