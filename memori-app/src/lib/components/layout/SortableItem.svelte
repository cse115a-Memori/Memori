<script lang="ts">
	import type { Data, Type, UniqueIdentifier } from '@dnd-kit/abstract'
	import { useSortable } from '@dnd-kit-svelte/svelte/sortable'
	import { onMount } from 'svelte'
	import type { ClassValue } from 'svelte/elements'
	import { prefsState } from '@/features/prefs/store.ts'
	import {
		kindToDisplay,
		type WidgetView,
	} from '@/features/widgets/model/widget-frame.ts'
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

	let { id, index, group, type, data, widget, cls, isOverlay = false }: Props = $props()
	const display = $derived(kindToDisplay(widget.kind))
	const isClock = $derived('Clock' in widget.kind)
	let now = $state(new Date())

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
</script>

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
		<div class={sortableCardTitleClasses}>{display.name}</div>
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
