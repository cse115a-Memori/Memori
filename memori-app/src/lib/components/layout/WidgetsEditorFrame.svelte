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
		type DragMutationEvent,
		findActiveWidget,
		getDragOverKey,
		getOverlaySize,
		projectLayoutFrame,
		shouldCancelOverflowSwapPreview,
		shouldResetDragPreviewOnOverflowMiss,
		shouldUseCommittedFrameForOverflowSwap,
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
	let isOverflowSwap = $state(false)
	let dragOverKey = $state<string | null>(null)
	const visibleFrame = $derived(previewFrame ?? frame)
	let overlayDims = $state<{ width: number; height: number } | null>(null)
	const overlayStyle = $derived(
		overlayDims
			? `width: ${overlayDims.width}px; height: ${overlayDims.height}px;`
			: undefined
	)
	let activeWidget = $state<WidgetView | null>(null)

	function getCurrentFrameSnapshot(): WidgetFrame {
		return $state.snapshot(frame) as WidgetFrame
	}

	function syncOverlaySize(op: Parameters<typeof getOverlaySize>[0]): void {
		const nextOverlayDims = getOverlaySize(op)
		if (nextOverlayDims) overlayDims = nextOverlayDims
	}

	function resetDragState(): void {
		previewFrame = null
		isOverflowSwap = false
		dragOverKey = null
		activeWidget = null
		overlayDims = null
	}

	function onFrameCommit(previewFrame: WidgetFrame) {
		setWidgetFrame(widgetsState.activeFrameIdx, previewFrame)
	}

	function projectDragPreview(evt: DragMutationEvent): {
		sourceFrame: WidgetFrame
		projectedLayout: WidgetFrame[keyof WidgetFrame] | null
		slotCount: number
		useCommittedFrame: boolean
	} {
		const slotCount = getLayoutSlotCount(layout)
		const committedFrame = getCurrentFrameSnapshot()
		const useCommittedFrame = shouldUseCommittedFrameForOverflowSwap(
			committedFrame[layout],
			evt,
			slotCount
		)
		const sourceFrame = useCommittedFrame
			? committedFrame
			: (previewFrame ?? committedFrame)
		const projectedLayout = projectLayoutFrame(sourceFrame[layout], evt, slotCount)

		return { sourceFrame, projectedLayout, slotCount, useCommittedFrame }
	}

	function commitDragResult(evt: DragMutationEvent): void {
		if (evt.operation.canceled) {
			resetDragState()
			return
		}

		const sourceFrame = getCurrentFrameSnapshot()
		const slotCount = getLayoutSlotCount(layout)
		if (isOverflowSwap) {
			// Overflow swap preview is computed from dragover snapshots and can be
			// valid even when dragend target metadata is unstable.
			if (previewFrame) {
				onFrameCommit(previewFrame)
				resetDragState()
				return
			}

			if (shouldCancelOverflowSwapPreview(sourceFrame[layout], evt, slotCount)) {
				resetDragState()
				return
			}

			const projectedLayout = projectLayoutFrame(sourceFrame[layout], evt, slotCount)
			if (projectedLayout) {
				onFrameCommit({ ...sourceFrame, [layout]: projectedLayout })
			}
		} else if (!evt.operation.target) {
			resetDragState()
			return
		} else if (previewFrame) {
			onFrameCommit(previewFrame)
		} else {
			const projectedLayout = projectLayoutFrame(sourceFrame[layout], evt, slotCount)
			if (projectedLayout) {
				onFrameCommit({ ...sourceFrame, [layout]: projectedLayout })
			}
		}

		resetDragState()
	}

	const handleDragStart: DragDropEvents['dragstart'] = evt => {
		const source = evt.operation.source
		if (!source) return

		previewFrame = getCurrentFrameSnapshot()
		isOverflowSwap = false
		activeWidget = findActiveWidget(previewFrame[layout], String(source.id))
		dragOverKey = null
		syncOverlaySize(evt.operation)
	}

	const handleDragOver: DragDropEvents['dragover'] = evt => {
		syncOverlaySize(evt.operation)
		const nextDragOverKey = getDragOverKey(evt.operation)
		if (nextDragOverKey && nextDragOverKey === dragOverKey) return

		const { sourceFrame, projectedLayout, slotCount, useCommittedFrame } =
			projectDragPreview(evt)
		if (!projectedLayout) {
			if (shouldResetDragPreviewOnOverflowMiss(sourceFrame[layout], evt, slotCount)) {
				previewFrame = null
				isOverflowSwap = false
			}
			dragOverKey = nextDragOverKey ?? null
			return
		}

		previewFrame = { ...sourceFrame, [layout]: projectedLayout }
		isOverflowSwap = useCommittedFrame
		dragOverKey = nextDragOverKey ?? null
	}

	const handleDragEnd: DragDropEvents['dragend'] = evt => {
		commitDragResult(evt)
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
