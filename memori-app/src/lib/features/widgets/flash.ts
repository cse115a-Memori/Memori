import { getLayoutSlotCount, type LayoutVariant } from '@/features/widgets/model/layout'
import type { WidgetsState } from '@/features/widgets/types'
import type { MemoriLayout, MemoriStateInput, MemoriWidget } from '@/tauri'

function encodeLayout(
	layout: LayoutVariant,
	widgetIds: Memori.WidgetId[]
): MemoriLayout {
	switch (layout) {
		case 'Full':
			return { Full: widgetIds[0] as Memori.WidgetId }
		case 'VSplit':
			return {
				VSplit: {
					left: widgetIds[0] as Memori.WidgetId,
					right: widgetIds[1] as Memori.WidgetId,
				},
			}
		case 'HSplit':
			return {
				HSplit: {
					top: widgetIds[0] as Memori.WidgetId,
					bottom: widgetIds[1] as Memori.WidgetId,
				},
			}
		case 'VSplitWithRightHSplit':
			return {
				VSplitWithRightHSplit: {
					left: widgetIds[0] as Memori.WidgetId,
					rightTop: widgetIds[1] as Memori.WidgetId,
					rightBottom: widgetIds[2] as Memori.WidgetId,
				},
			}
		case 'HSplitWithTopVSplit':
			return {
				HSplitWithTopVSplit: {
					topLeft: widgetIds[0] as Memori.WidgetId,
					topRight: widgetIds[1] as Memori.WidgetId,
					bottom: widgetIds[2] as Memori.WidgetId,
				},
			}
		case 'VSplitWithLeftHSplit':
			return {
				VSplitWithLeftHSplit: {
					leftTop: widgetIds[0] as Memori.WidgetId,
					leftBottom: widgetIds[1] as Memori.WidgetId,
					right: widgetIds[2] as Memori.WidgetId,
				},
			}
		case 'HSplitWithBottomVSplit':
			return {
				HSplitWithBottomVSplit: {
					top: widgetIds[0] as Memori.WidgetId,
					bottomLeft: widgetIds[1] as Memori.WidgetId,
					bottomRight: widgetIds[2] as Memori.WidgetId,
				},
			}
		case 'Fourths':
			return {
				Fourths: {
					topLeft: widgetIds[0] as Memori.WidgetId,
					topRight: widgetIds[1] as Memori.WidgetId,
					bottomLeft: widgetIds[2] as Memori.WidgetId,
					bottomRight: widgetIds[3] as Memori.WidgetId,
				},
			}
	}
}

export function validateWidgetsStateForFlash(state: WidgetsState): string | null {
	if (state.frames.length === 0) {
		return 'Cannot flash: no frames are configured.'
	}

	const widgetIds = new Set(state.widgets.map(widget => widget.id))
	if (widgetIds.size === 0) {
		return 'Cannot flash: no widgets are available.'
	}

	for (const [idx, frame] of state.frames.entries()) {
		const layout = frame.activeLayout
		const assignments = frame.layoutAssignments[layout] ?? []
		const expectedSlotsLen = getLayoutSlotCount(layout)

		if (assignments.length !== expectedSlotsLen) {
			return `Cannot flash: frame ${idx + 1} (${layout}) requires ${expectedSlotsLen} widget slot${expectedSlotsLen === 1 ? '' : 's'}, but has ${assignments.length}.`
		}

		const assignmentIds = assignments.map(widget => widget.id)
		const uniqueAssignmentIds = new Set(assignmentIds)
		if (uniqueAssignmentIds.size !== assignmentIds.length) {
			return `Cannot flash: frame ${idx + 1} (${layout}) contains duplicate widget assignments.`
		}

		for (const widgetId of assignmentIds) {
			if (!widgetIds.has(widgetId)) {
				return `Cannot flash: frame ${idx + 1} references widget ${widgetId}, but it is missing from the widget pool.`
			}
		}
	}

	return null
}

export function selectFlashPayload(state: WidgetsState): MemoriStateInput {
	// const widgets = [...state.widgets]
	let widgets: MemoriWidget[] = []

	const frames = state.frames.map(frame => {
		const layout = frame.activeLayout
		const widgetIds = (frame.layoutAssignments[layout] ?? []).map(widget => widget.id)
		console.log('widgetIds', widgetIds)
		widgets = state.widgets.filter(w => widgetIds.includes(w.id))
		return encodeLayout(layout, widgetIds)
	})

	const clampedActiveFrameIdx =
		frames.length === 0
			? 0
			: Math.min(Math.max(0, Math.trunc(state.activeFrameIdx)), frames.length - 1)

	return {
		activeFrameIdx: clampedActiveFrameIdx,
		widgets,
		frames,
		frameTime: state.frameTime,
	}
}
