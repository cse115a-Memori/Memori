<script lang="ts">
	import {
		type DragDropEvents,
		DragDropProvider,
		DragOverlay,
	} from '@dnd-kit-svelte/svelte'
	import { RestrictToWindowEdges } from '@dnd-kit-svelte/svelte/modifiers'
	import { onMount } from 'svelte'
	import {
		formatCompactClock,
		WidgetSection,
		WidgetsDragOverlay,
		WidgetsToolbar,
	} from '@/components/layout'
	import { prefsState } from '@/features/prefs/store.ts'
	import {
		ensureWidgetFrameEntry,
		setWidgetFrame,
		setWidgetFrameLayout,
		syncWidgetPoolAndFrame,
		widgetsEditorState,
	} from '@/features/widgets/editor-store.ts'
	import {
		memoriDraftState,
		setMemoriDraft,
	} from '@/features/widgets/memori-draft-store.ts'
	import {
		getLayoutSlotCount,
		LAYOUT_TEMPLATES,
	} from '@/features/widgets/model/layout.ts'
	import {
		findActiveWidget,
		getDragOverKey,
		projectLayoutFrameOnDragOver,
		resolveOverlaySize,
		shouldCancelOverflowSwapPreview,
		shouldResetDragPreviewOnOverflowMiss,
		shouldUseCommittedFrameForOverflowSwap,
	} from '@/features/widgets/model/widget-dnd.ts'
	import {
		createWidgetFrameEntry,
		getFrameWidgetCount,
		isLayoutVariant,
		type WidgetFrame,
		type WidgetFrameEntry,
		type WidgetView,
	} from '@/features/widgets/model/widget-frame.ts'
	import { getWidgetKinds } from '@/features/widgets/service.ts'
	import {
		commands,
		type MemoriLayout,
		type MemoriStateInput,
		type MemoriWidget,
		tryCmd,
	} from '@/tauri'
	import { sensors } from '$lib'

	let errMsg = $state('')
	let currFrameIdx = $state(0)
	let now = $state(new Date())
	const FLASH_DURATION_MS = 1000
	const CLOCK_TICK_MS = 1000
	const FRAME_TIME_SECONDS = 5

	const toMemoriLayout = (
		layout: Memori.Layout['variant'],
		frameWidgets: WidgetView[]
	): MemoriLayout => {
		switch (layout) {
			case 'Full':
				return { Full: frameWidgets[0].widgetId }
			case 'VSplit':
				return {
					VSplit: {
						left: frameWidgets[0].widgetId,
						right: frameWidgets[1].widgetId,
					},
				}
			case 'HSplit':
				return {
					HSplit: {
						top: frameWidgets[0].widgetId,
						bottom: frameWidgets[1].widgetId,
					},
				}
			case 'VSplitWithRightHSplit':
				return {
					VSplitWithRightHSplit: {
						left: frameWidgets[0].widgetId,
						rightTop: frameWidgets[1].widgetId,
						rightBottom: frameWidgets[2].widgetId,
					},
				}
			case 'HSplitWithTopVSplit':
				return {
					HSplitWithTopVSplit: {
						topLeft: frameWidgets[0].widgetId,
						topRight: frameWidgets[1].widgetId,
						bottom: frameWidgets[2].widgetId,
					},
				}
			case 'VSplitWithLeftHSplit':
				return {
					VSplitWithLeftHSplit: {
						leftTop: frameWidgets[0].widgetId,
						leftBottom: frameWidgets[1].widgetId,
						right: frameWidgets[2].widgetId,
					},
				}
			case 'HSplitWithBottomVSplit':
				return {
					HSplitWithBottomVSplit: {
						top: frameWidgets[0].widgetId,
						bottomLeft: frameWidgets[1].widgetId,
						bottomRight: frameWidgets[2].widgetId,
					},
				}
			case 'Fourths':
				return {
					Fourths: {
						topLeft: frameWidgets[0].widgetId,
						topRight: frameWidgets[1].widgetId,
						bottomLeft: frameWidgets[2].widgetId,
						bottomRight: frameWidgets[3].widgetId,
					},
				}
		}
	}

	let isFlashing = $state(false)
	const flash = async () => {
		const frameEntries = widgetsEditorState.widgetFrames.map((_, frameIdx) =>
			ensureWidgetFrameEntry(frameIdx)
		)
		if (frameEntries.length === 0) {
			errMsg = 'Cannot flash: no frames are configured.'
			return
		}

		const framesPayload: MemoriLayout[] = []
		for (const [frameIdx, frameEntry] of frameEntries.entries()) {
			const layout = frameEntry.activeLayout
			const expectedSlotsLen = getLayoutSlotCount(layout)
			const actualSlotsLen = getFrameWidgetCount(frameEntry.frameLayouts, layout)
			if (actualSlotsLen !== expectedSlotsLen) {
				errMsg = `Cannot flash: frame ${frameIdx + 1} (${layout}) requires ${expectedSlotsLen} widget slot${expectedSlotsLen === 1 ? '' : 's'}, but has ${actualSlotsLen}.`
				return
			}

			framesPayload.push(
				toMemoriLayout(layout, frameEntry.frameLayouts[layout]['frame-widgets'])
			)
		}

		const widgetsPayload = $state.snapshot(
			widgetsEditorState.widgetPool
		) as MemoriWidget[]
		const widgetIds = new Set(widgetsPayload.map(widget => widget.id))
		for (const [frameIdx, frameEntry] of frameEntries.entries()) {
			const layout = frameEntry.activeLayout
			for (const widget of frameEntry.frameLayouts[layout]['frame-widgets']) {
				if (!widgetIds.has(widget.widgetId)) {
					errMsg = `Cannot flash: frame ${frameIdx + 1} references widget ${widget.widgetId}, but it is missing from the widget pool.`
					return
				}
			}
		}

		errMsg = ''
		isFlashing = true

		const payload: MemoriStateInput = {
			activeFrameIdx: Math.min(currFrameIdx, frameEntries.length - 1),
			widgets: widgetsPayload,
			frames: framesPayload,
			frameTime: FRAME_TIME_SECONDS,
		}

		setMemoriDraft(payload)
		const draft = memoriDraftState.draft
		if (!draft) {
			errMsg = 'Flash failed: missing memori draft payload.'
			isFlashing = false
			return
		}

		await tryCmd(commands.setMemoriState(draft)).match(
			() => {
				errMsg = ''
			},
			error => {
				errMsg = `Flash failed: ${error}`
			}
		)
		isFlashing = false
	}

	const getCurrentFrameSnapshot = (): WidgetFrame =>
		$state.snapshot(currWidgetFrame) as WidgetFrame

	const loadWidgets = (): Promise<void> =>
		getWidgetKinds().match(
			data => {
				syncWidgetPoolAndFrame(currFrameIdx, data)
			},
			error => {
				errMsg = `Load widgets failed: ${error}`
			}
		)

	const compactClock = $derived(
		formatCompactClock(now, prefsState.systemOptions.timeZone ?? undefined)
	)

	onMount(() => {
		void loadWidgets()

		const intervalId = setInterval(() => {
			now = new Date()
		}, CLOCK_TICK_MS)

		return () => {
			clearInterval(intervalId)
		}
	})

	const defaultFrameEntry = createWidgetFrameEntry()
	$effect(() => {
		ensureWidgetFrameEntry(currFrameIdx)
	})
	const currFrameEntry = $derived(
		widgetsEditorState.widgetFrames[currFrameIdx] ?? defaultFrameEntry
	)
	const currLayout = $derived(currFrameEntry.activeLayout)
	const currWidgetFrame = $derived(currFrameEntry.frameLayouts)
	const tmpl = $derived(LAYOUT_TEMPLATES[currLayout])

	let dragFrame = $state<WidgetFrame | null>(null)
	let hasOverflowSwapPreview = $state(false)
	let lastDragOverKey = $state<string | null>(null)
	const visibleFrame = $derived(dragFrame ?? currWidgetFrame)
	let overlaySize = $state<{ width: number; height: number } | null>(null)
	const overlayStyle = $derived(
		overlaySize
			? `width: ${overlaySize.width}px; height: ${overlaySize.height}px;`
			: undefined
	)

	const handleLayoutChange = (nextLayout: string) => {
		if (!isLayoutVariant(nextLayout)) return
		setWidgetFrameLayout(currFrameIdx, nextLayout)
		dragFrame = null
		lastDragOverKey = null
	}

	let activeWidget = $state<WidgetView | null>(null)

	function resetDragState(): void {
		dragFrame = null
		hasOverflowSwapPreview = false
		lastDragOverKey = null
		activeWidget = null
		overlaySize = null
	}

	function commitFrame(nextFrame: WidgetFrame): void {
		setWidgetFrame(currFrameIdx, nextFrame)
	}

	const handleDragStart: DragDropEvents['dragstart'] = event => {
		const source = event.operation.source
		if (!source) return

		dragFrame = getCurrentFrameSnapshot()
		hasOverflowSwapPreview = false
		activeWidget = findActiveWidget(dragFrame[currLayout], String(source.id))
		lastDragOverKey = null

		const nextOverlaySize = resolveOverlaySize(event.operation)
		if (nextOverlaySize) overlaySize = nextOverlaySize
	}

	const handleDragOver: DragDropEvents['dragover'] = event => {
		const nextOverlaySize = resolveOverlaySize(event.operation)
		if (nextOverlaySize) overlaySize = nextOverlaySize

		const nextKey = getDragOverKey(event.operation)
		if (nextKey && nextKey === lastDragOverKey) return

		const frameSlotCount = getLayoutSlotCount(currLayout)
		const committedFrame = getCurrentFrameSnapshot()
		const useCommittedFrame = shouldUseCommittedFrameForOverflowSwap(
			committedFrame[currLayout],
			event,
			frameSlotCount
		)
		const baseFrame = useCommittedFrame ? committedFrame : (dragFrame ?? committedFrame)
		const projected = projectLayoutFrameOnDragOver(
			baseFrame[currLayout],
			event,
			frameSlotCount
		)
		if (!projected) {
			if (
				shouldResetDragPreviewOnOverflowMiss(
					baseFrame[currLayout],
					event,
					frameSlotCount
				)
			) {
				dragFrame = null
				hasOverflowSwapPreview = false
			}

			lastDragOverKey = nextKey ?? null
			return
		}

		dragFrame = { ...baseFrame, [currLayout]: projected }
		hasOverflowSwapPreview = useCommittedFrame
		lastDragOverKey = nextKey ?? null
	}

	const handleDragEnd: DragDropEvents['dragend'] = event => {
		if (event.operation.canceled) {
			resetDragState()
			return
		}

		if (!event.operation.target) {
			resetDragState()
			return
		}

		const baseFrame = getCurrentFrameSnapshot()
		const frameSlotCount = getLayoutSlotCount(currLayout)
		if (hasOverflowSwapPreview) {
			if (
				shouldCancelOverflowSwapPreview(baseFrame[currLayout], event, frameSlotCount)
			) {
				resetDragState()
				return
			}

			const projected = projectLayoutFrameOnDragOver(
				baseFrame[currLayout],
				event,
				frameSlotCount
			)
			if (projected) {
				commitFrame({ ...baseFrame, [currLayout]: projected })
			}
		} else if (dragFrame) {
			commitFrame(dragFrame)
		} else {
			const projected = projectLayoutFrameOnDragOver(
				baseFrame[currLayout],
				event,
				frameSlotCount
			)
			if (projected) {
				commitFrame({ ...baseFrame, [currLayout]: projected })
			}
		}

		resetDragState()
	}
</script>

<WidgetsToolbar
	layout={currLayout}
	{isFlashing}
	onLayoutChange={handleLayoutChange}
	onFlash={flash}
/>

{#if errMsg}
	<p class="text-sm text-red-600">{errMsg}</p>
{/if}

<DragDropProvider
	{sensors}
	modifiers={[RestrictToWindowEdges]}
	onDragStart={handleDragStart}
	onDragOver={handleDragOver}
	onDragEnd={handleDragEnd}
>
	<div class="grid gap-2 md:grid-cols-[1fr_3fr]">
		<WidgetSection
			id="widgets"
			title="Widgets"
			tasks={visibleFrame[currLayout]['widgets']}
			layout={currLayout}
			frameContainerClass={tmpl.container}
		/>
		<WidgetSection
			id="frame-widgets"
			title="Frame"
			tasks={visibleFrame[currLayout]['frame-widgets']}
			layout={currLayout}
			frameContainerClass={tmpl.container}
		/>
	</div>

	<DragOverlay>
		{#snippet children(_source)}
			<WidgetsDragOverlay {activeWidget} {overlayStyle} {compactClock} />
		{/snippet}
	</DragOverlay>
</DragDropProvider>
