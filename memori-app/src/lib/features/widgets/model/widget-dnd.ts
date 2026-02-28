import type { DragOperation } from '@dnd-kit/abstract'
import { move } from '@dnd-kit/helpers'
import type { DragDropEvents } from '@dnd-kit-svelte/svelte'
import {
	type GroupId,
	isGroupId,
	type WidgetFrame,
	type WidgetView,
} from '@/features/widgets/model/widget-frame.ts'

type LayoutFrame = WidgetFrame[keyof WidgetFrame]

type DragItem = {
	id?: unknown
	data?: unknown
	index?: unknown
	shape?: {
		center?: { y?: number }
		boundingRectangle?: { top?: number; height?: number }
	}
	manager?: {
		dragOperation?: {
			shape?: { current?: { center?: { y?: number } } }
			position?: { current?: { y?: number } }
		}
	}
}

export type DragMutationEvent = Parameters<
	DragDropEvents['dragover'] | DragDropEvents['dragend']
>[0]

function findIndexById(items: WidgetView[], id: string): number {
	return items.findIndex(entry => entry.id === id)
}

function getGroupFromData(data: unknown): GroupId | undefined {
	if (!data || typeof data !== 'object') return undefined
	if (!('group' in data)) return undefined
	return isGroupId((data as { group?: unknown }).group)
		? (data as { group: GroupId }).group
		: undefined
}

function getGroupIdFromDragItem(
	item: { id?: unknown; data?: unknown } | null | undefined
): GroupId | undefined {
	const dataGroup = getGroupFromData(item?.data)
	if (dataGroup) return dataGroup
	return isGroupId(item?.id) ? item.id : undefined
}

function isPoolSourceWithFullFrame(
	layoutFrame: LayoutFrame,
	source: DragItem | undefined,
	frameSlotCount: number | undefined
): boolean {
	if (typeof frameSlotCount !== 'number') return false

	const sourceGroup = getGroupIdFromDragItem(source)
	const isFrameFull = layoutFrame['frame-widgets'].length >= frameSlotCount

	return isFrameFull && sourceGroup === 'widgets'
}

function getFrameTargetIndex(
	layoutFrame: LayoutFrame,
	target: DragItem | undefined,
	event?: DragMutationEvent
): number {
	const frameLength = layoutFrame['frame-widgets'].length
	const targetId = target?.id
	if (targetId !== undefined) {
		const explicitIndex = findIndexById(layoutFrame['frame-widgets'], String(targetId))
		if (explicitIndex >= 0) return explicitIndex
	}

	const targetGroup = getGroupIdFromDragItem(target)
	if (targetGroup !== 'frame-widgets') return -1

	const targetIndexFromPayload =
		typeof target?.index === 'number' ? target.index : undefined
	if (typeof targetIndexFromPayload === 'number' && targetIndexFromPayload >= 0) {
		if (frameLength <= 0) return -1
		return Math.min(targetIndexFromPayload, frameLength - 1)
	}

	const source = event?.operation?.source as DragItem | undefined
	const pointerY =
		source?.manager?.dragOperation?.shape?.current?.center?.y ??
		source?.manager?.dragOperation?.position?.current?.y ??
		event?.operation?.source?.manager?.dragOperation?.position?.current?.y ??
		event?.operation?.source?.manager?.dragOperation?.shape?.current?.center?.y ??
		event?.operation?.target?.shape?.center?.y
	const targetRect = target?.shape?.boundingRectangle

	if (
		frameLength === 0 ||
		pointerY === undefined ||
		!targetRect ||
		typeof targetRect.top !== 'number' ||
		typeof targetRect.height !== 'number' ||
		targetRect.height <= 0
	) {
		return frameLength > 0 ? 0 : -1
	}

	const normalized = (pointerY - targetRect.top) / targetRect.height
	if (!Number.isFinite(normalized)) return 0

	const clamped = Math.min(Math.max(normalized, 0), 1)
	return Math.min(frameLength - 1, Math.max(0, Math.floor(clamped * frameLength)))
}

export function findActiveWidget(
	layoutFrame: LayoutFrame,
	sourceId: string
): WidgetView | null {
	return (
		layoutFrame.widgets.find(entry => entry.id === sourceId) ??
		layoutFrame['frame-widgets'].find(entry => entry.id === sourceId) ??
		null
	)
}

export function projectLayoutFrameOnDragOver(
	layoutFrame: LayoutFrame,
	event: DragMutationEvent,
	frameSlotCount?: number
): LayoutFrame | null {
	const source = event.operation.source as DragItem | undefined
	if (!source) return null

	const sourceId = String(source.id)
	const hasSource =
		findIndexById(layoutFrame.widgets, sourceId) >= 0 ||
		findIndexById(layoutFrame['frame-widgets'], sourceId) >= 0
	if (!hasSource) return null

	const target = event.operation.target as DragItem | undefined

	if (isPoolSourceWithFullFrame(layoutFrame, source, frameSlotCount)) {
		const targetGroup = getGroupIdFromDragItem(target)
		if (targetGroup !== 'frame-widgets') return null

		const poolIndex = findIndexById(layoutFrame.widgets, sourceId)
		const frameIndex = getFrameTargetIndex(layoutFrame, target, event)
		if (frameIndex < 0) return null
		if (poolIndex < 0) return null

		const sourceWidget = layoutFrame.widgets[poolIndex]
		const replacedWidget = layoutFrame['frame-widgets'][frameIndex]
		const nextPool = [
			...layoutFrame.widgets.slice(0, poolIndex),
			...layoutFrame.widgets.slice(poolIndex + 1),
			replacedWidget,
		]
		const nextFrameWidgets = [...layoutFrame['frame-widgets']]
		nextFrameWidgets[frameIndex] = sourceWidget

		return {
			...layoutFrame,
			widgets: nextPool,
			'frame-widgets': nextFrameWidgets,
		}
	}

	const moved = move(layoutFrame, event)
	return moved === layoutFrame ? null : moved
}

export function shouldUseCommittedFrameForOverflowSwap(
	layoutFrame: LayoutFrame,
	event: DragMutationEvent,
	frameSlotCount: number
): boolean {
	const source = event.operation.source as DragItem | undefined
	const target = event.operation.target as DragItem | undefined
	if (!isPoolSourceWithFullFrame(layoutFrame, source, frameSlotCount)) {
		return false
	}

	return getGroupIdFromDragItem(target) === 'frame-widgets'
}

export function shouldCancelOverflowSwapPreview(
	layoutFrame: LayoutFrame,
	event: DragMutationEvent,
	frameSlotCount: number
): boolean {
	return !shouldUseCommittedFrameForOverflowSwap(layoutFrame, event, frameSlotCount)
}

export function shouldResetDragPreviewOnOverflowMiss(
	layoutFrame: LayoutFrame,
	event: DragMutationEvent,
	frameSlotCount: number
): boolean {
	const target = event.operation.target as DragItem | undefined
	if (
		!isPoolSourceWithFullFrame(
			layoutFrame,
			event.operation.source as DragItem | undefined,
			frameSlotCount
		)
	) {
		return false
	}

	const targetGroup = getGroupIdFromDragItem(target)
	if (targetGroup === 'widgets') return true
	if (targetGroup !== 'frame-widgets') return false
	return getFrameTargetIndex(layoutFrame, target, event) < 0
}

export function resolveOverlaySize(
	operation: DragOperation | undefined
): { width: number; height: number } | null {
	const targetId = operation?.target?.id
	const isItemTarget =
		targetId !== undefined && !isGroupId(typeof targetId === 'string' ? targetId : '')
	const targetRect = isItemTarget
		? operation?.target?.shape?.boundingRectangle
		: undefined
	const dragRect = operation?.shape?.current?.boundingRectangle
	const nextRect = targetRect ?? dragRect
	if (!nextRect) return null

	return { width: nextRect.width, height: nextRect.height }
}

export function getOverlaySize(
	operation: DragOperation | undefined
): { width: number; height: number } | null {
	const targetId = operation?.target?.id
	const isItemTarget =
		targetId !== undefined && !isGroupId(typeof targetId === 'string' ? targetId : '')
	const targetRect = isItemTarget
		? operation?.target?.shape?.boundingRectangle
		: undefined
	const dragRect = operation?.shape?.current?.boundingRectangle
	const nextRect = targetRect ?? dragRect
	if (!nextRect) return null

	return { width: nextRect.width, height: nextRect.height }
}

export function getDragOverKey(operation: DragOperation | undefined): string | null {
	const source = operation?.source as DragItem | undefined
	const target = operation?.target as DragItem | undefined
	const sourceGroup = getGroupIdFromDragItem(source)
	const targetGroup = getGroupIdFromDragItem(target)

	if (!source || !target || !sourceGroup || !targetGroup) return null

	const sourceIndex = typeof source.index === 'number' ? source.index : -1
	const targetIndex = typeof target.index === 'number' ? target.index : -1

	const operationShape = source.manager?.dragOperation?.shape?.current
	const operationPosition = source.manager?.dragOperation?.position?.current
	const pointerY = operationShape?.center?.y ?? operationPosition?.y
	const targetY = target.shape?.center?.y
	const side =
		pointerY !== undefined && targetY !== undefined
			? pointerY > targetY
				? 'after'
				: 'before'
			: 'none'

	return `${String(source.id)}:${sourceGroup}:${sourceIndex}->${String(target.id)}:${targetGroup}:${targetIndex}:${side}`
}
