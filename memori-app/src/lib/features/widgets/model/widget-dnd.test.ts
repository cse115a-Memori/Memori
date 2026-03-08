// @ts-nocheck
import { describe, expect, test } from 'bun:test'
import {
	createEmptyWidgetFrame,
	type WidgetFrame,
	type WidgetView,
} from './widget-frame.ts'
import {
	projectWidgetFrame,
	type DragMutationEvent,
} from './widget-dnd.ts'

function createWidgetView(id: string, widgetId: number): WidgetView {
	return {
		id,
		widgetId,
		kind: { Name: { name: id } },
	}
}

function createFrame(
	layout: keyof WidgetFrame,
	widgets: WidgetView[],
	frameWidgets: WidgetView[]
): WidgetFrame {
	const frame = createEmptyWidgetFrame()
	frame[layout] = {
		widgets,
		'frame-widgets': frameWidgets,
	}
	return frame
}

function createDragEvent({
	sourceId,
	sourceGroup,
	sourceIndex,
	targetId,
		targetGroup,
		targetIndex,
	}: {
		sourceId: string
		sourceGroup: 'widgets' | 'frame-widgets'
		sourceIndex: number
		targetId: string
		targetGroup: 'widgets' | 'frame-widgets'
		targetIndex: number
	}): DragMutationEvent {
	return {
		operation: {
			canceled: false,
			source: {
				id: sourceId,
				data: { group: sourceGroup, index: sourceIndex },
				index: sourceIndex,
				manager: {
					dragOperation: {
						position: { current: { x: 0, y: 0 } },
					},
				},
			},
			target: {
				id: targetId,
				data: { group: targetGroup, index: targetIndex },
				index: targetIndex,
				shape: {
					center: { x: 0, y: 0 },
				},
			},
		},
	} as DragMutationEvent
}

describe('projectWidgetFrame', () => {
	test('uses the latest projected frame across sequential hover targets', () => {
		const widgetA = createWidgetView('pool-a', 1)
		const widgetB = createWidgetView('pool-b', 2)
		const widgetC = createWidgetView('pool-c', 3)
		const initialFrame = createFrame('VSplit', [widgetA, widgetB, widgetC], [])

		const firstHover = createDragEvent({
			sourceId: widgetA.id,
			sourceGroup: 'widgets',
			sourceIndex: 0,
			targetId: widgetB.id,
			targetGroup: 'widgets',
			targetIndex: 1,
		})
		const firstProjectedFrame = projectWidgetFrame(initialFrame, firstHover, 'VSplit', 2)
		expect(firstProjectedFrame).not.toBeNull()
		expect(firstProjectedFrame?.VSplit.widgets.map(widget => widget.id)).toEqual([
			widgetB.id,
			widgetA.id,
			widgetC.id,
		])

		const secondHover = createDragEvent({
			sourceId: widgetA.id,
			sourceGroup: 'widgets',
			sourceIndex: 1,
			targetId: widgetC.id,
			targetGroup: 'widgets',
			targetIndex: 2,
		})
		const secondProjectedFrame = projectWidgetFrame(
			firstProjectedFrame as WidgetFrame,
			secondHover,
			'VSplit',
			2
		)
		expect(secondProjectedFrame).not.toBeNull()
		expect(secondProjectedFrame?.VSplit.widgets.map(widget => widget.id)).toEqual([
			widgetB.id,
			widgetC.id,
			widgetA.id,
		])
	})

	test('supports overflow swap across multiple hovered frame slots', () => {
		const poolWidget = createWidgetView('pool-x', 10)
		const frameWidgetA = createWidgetView('frame-a', 11)
		const frameWidgetB = createWidgetView('frame-b', 12)
		const initialFrame = createFrame('VSplit', [poolWidget], [frameWidgetA, frameWidgetB])

		const firstHover = createDragEvent({
			sourceId: poolWidget.id,
			sourceGroup: 'widgets',
			sourceIndex: 0,
			targetId: frameWidgetA.id,
			targetGroup: 'frame-widgets',
			targetIndex: 0,
		})
		const firstProjectedFrame = projectWidgetFrame(initialFrame, firstHover, 'VSplit', 2)
		expect(firstProjectedFrame).not.toBeNull()
		expect(firstProjectedFrame?.VSplit['frame-widgets'].map(widget => widget.id)).toEqual([
			poolWidget.id,
			frameWidgetB.id,
		])
		expect(firstProjectedFrame?.VSplit.widgets.map(widget => widget.id)).toEqual([
			frameWidgetA.id,
		])

		const secondHover = createDragEvent({
			sourceId: poolWidget.id,
			sourceGroup: 'widgets',
			sourceIndex: 0,
			targetId: frameWidgetB.id,
			targetGroup: 'frame-widgets',
			targetIndex: 1,
		})
		const secondProjectedFrame = projectWidgetFrame(
			firstProjectedFrame as WidgetFrame,
			secondHover,
			'VSplit',
			2
		)
		expect(secondProjectedFrame).not.toBeNull()
		expect(secondProjectedFrame?.VSplit['frame-widgets'].map(widget => widget.id)).toEqual([
			frameWidgetB.id,
			poolWidget.id,
		])
		expect(secondProjectedFrame?.VSplit.widgets.map(widget => widget.id)).toEqual([
			frameWidgetA.id,
		])
	})
})
