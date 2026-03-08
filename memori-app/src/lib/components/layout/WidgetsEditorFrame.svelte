<script lang="ts">
	import {
		type DragDropEvents,
		DragDropProvider,
		DragOverlay,
	} from '@dnd-kit-svelte/svelte'
	import { RestrictToWindowEdges } from '@dnd-kit-svelte/svelte/modifiers'
	import {
		getLayoutSlotCount,
		LAYOUT_TEMPLATES,
		type LayoutVariant,
	} from '@/features/widgets/model/layout'
	import {
		findActiveWidget,
		getDragOverKey,
		getOverlaySize,
		projectWidgetFrame,
	} from '@/features/widgets/model/widget-dnd'
	import {
		type WidgetFrame,
		type WidgetView,
	} from '@/features/widgets/model/widget-frame'
	import { setWidgetFrame, widgetsState } from '@/features/widgets/widgets-store'
	import { sensors } from '$lib'
	import WidgetSection from './WidgetSection.svelte'
	import WidgetsDragOverlay from './WidgetsDragOverlay.svelte'
	import type { CompactClock } from './widget-clock'

	interface Props {
		layout: LayoutVariant
		frame: WidgetFrame
		compactClock: CompactClock
	}

	let { layout, frame, compactClock }: Props = $props()
	const layoutTpl = $derived(LAYOUT_TEMPLATES[layout])

	let previewFrame = $state<WidgetFrame | null>(null)
	let dragOverKey = $state<string | null>(null)
	let overlayDims = $state<{ width: number; height: number } | null>(null)
	const visibleFrame = $derived(previewFrame ?? frame)
	const overlayStyle = $derived(
		overlayDims
			? `width: ${overlayDims.width}px; height: ${overlayDims.height}px;`
			: undefined
	)
	let activeWidget = $state<WidgetView | null>(null)

	function syncOverlaySize(op: Parameters<typeof getOverlaySize>[0]): void {
		const nextOverlayDims = getOverlaySize(op)
		if (nextOverlayDims) overlayDims = nextOverlayDims
	}

	function resetDragState(): void {
		previewFrame = null
		dragOverKey = null
		activeWidget = null
		overlayDims = null
	}

	const handleDragStart: DragDropEvents['dragstart'] = evt => {
		const source = evt.operation.source
		if (!source) return

		previewFrame = null
		activeWidget = findActiveWidget(frame[layout], String(source.id))
		dragOverKey = null
		syncOverlaySize(evt.operation)
	}

	const handleDragOver: DragDropEvents['dragover'] = evt => {
		syncOverlaySize(evt.operation)
		const nextDragOverKey = getDragOverKey(evt.operation)
		if (nextDragOverKey && nextDragOverKey === dragOverKey) return

		const sourceFrame = previewFrame ?? frame
		const nextFrame = projectWidgetFrame(
			sourceFrame,
			evt,
			layout,
			getLayoutSlotCount(layout)
		)
		if (nextFrame) {
			previewFrame = nextFrame
		}

		dragOverKey = nextDragOverKey ?? null
	}

	const handleDragEnd: DragDropEvents['dragend'] = evt => {
		if (!evt.operation.canceled) {
			const finalFrame =
				previewFrame ??
				projectWidgetFrame(frame, evt, layout, getLayoutSlotCount(layout))
			if (finalFrame) {
				setWidgetFrame(widgetsState.activeFrameIdx, finalFrame)
			}
		}

		resetDragState()
	}
</script>

<DragDropProvider
	{sensors}
	modifiers={[RestrictToWindowEdges]}
	onDragStart={handleDragStart}
	onDragOver={handleDragOver}
	onDragEnd={handleDragEnd}
>
		<div class="grid gap-2 md:grid-cols-[3fr_1fr]">
			<WidgetSection
				id="frame-widgets"
				title="Frame"
				widgets={visibleFrame[layout]['frame-widgets']}
				{layout}
				frameContainerClass={layoutTpl.container}
			/>
			<WidgetSection
				id="widgets"
				title="Widgets"
				widgets={visibleFrame[layout]['widgets']}
				{layout}
				frameContainerClass={layoutTpl.container}
			/>
	</div>

	<DragOverlay>
		{#snippet children(_source)}
			<WidgetsDragOverlay {activeWidget} {overlayStyle} {compactClock} />
		{/snippet}
	</DragOverlay>
</DragDropProvider>
