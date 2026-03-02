import type { DragOperation } from '@dnd-kit/abstract'
import { move } from '@dnd-kit/helpers'
import type { DragDropEvents } from '@dnd-kit-svelte/svelte'
import {
	type GroupId,
	isGroupId,
	type WidgetFrame,
	type WidgetView,
} from '@/features/widgets/model/widget-frame.ts'

type FrameState = WidgetFrame[keyof WidgetFrame]
type PointerPos = { x?: number; y?: number }

type DragItem = {
	id?: unknown
	data?: unknown
	index?: unknown
	shape?: {
		center?: PointerPos
		boundingRectangle?: {
			top?: number
			left?: number
			right?: number
			bottom?: number
			height: number
			width: number
		}
	}
	manager?: {
		dragOperation?: {
			shape?: { current?: { center?: PointerPos } }
			position?: { current?: PointerPos }
		}
		registry?: { droppables?: Iterable<DragItem> }
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

function getIndexFromData(data: unknown): number | undefined {
	if (!data || typeof data !== 'object') return undefined
	if (!('index' in data)) return undefined
	const value = (data as { index?: unknown }).index
	if (typeof value !== 'number' || !Number.isFinite(value)) return undefined
	return Math.trunc(value)
}

function getGroupIdFromDragItem(
	item: { id?: unknown; data?: unknown } | null | undefined
): GroupId | undefined {
	const groupFromData = getGroupFromData(item?.data)
	if (groupFromData) return groupFromData
	return isGroupId(item?.id) ? item.id : undefined
}

function getSource(op: DragOperation | undefined): DragItem | undefined {
	return op?.source as DragItem | undefined
}

function getTarget(op: DragOperation | undefined): DragItem | undefined {
	return op?.target as DragItem | undefined
}

function getPointer(
	source: DragItem | undefined,
	target: DragItem | undefined
): PointerPos | null {
	const fromShape = source?.manager?.dragOperation?.shape?.current?.center
	if (typeof fromShape?.x === 'number' && typeof fromShape?.y === 'number') {
		return fromShape
	}

	const fromPos = source?.manager?.dragOperation?.position?.current
	if (typeof fromPos?.x === 'number' && typeof fromPos?.y === 'number') {
		return fromPos
	}

	const fromTarget = target?.shape?.center
	if (typeof fromTarget?.x === 'number' && typeof fromTarget?.y === 'number') {
		return fromTarget
	}

	return null
}

function isPoolSourceWithFullFrame(
	frameState: FrameState,
	source: DragItem | undefined,
	frameSlotCount: number | undefined
): boolean {
	if (typeof frameSlotCount !== 'number') return false

	const srcGroup = getGroupIdFromDragItem(source)
	const isFrameFull = frameState['frame-widgets'].length >= frameSlotCount
	return isFrameFull && srcGroup === 'widgets'
}

function isPointInsideBounds(
	boundingRectangle:
		| {
				top?: number
				left?: number
				right?: number
				bottom?: number
				height: number
				width: number
		  }
		| undefined,
	pointer: { x: number; y: number }
): boolean {
	if (!boundingRectangle) return false

	const top = boundingRectangle.top
	const left =
		typeof boundingRectangle.left === 'number'
			? boundingRectangle.left
			: typeof boundingRectangle.right === 'number'
				? boundingRectangle.right - boundingRectangle.width
				: undefined
	if (typeof top !== 'number' || typeof left !== 'number') return false

	const right =
		typeof boundingRectangle.right === 'number'
			? boundingRectangle.right
			: left + boundingRectangle.width
	const bottom =
		typeof boundingRectangle.bottom === 'number'
			? boundingRectangle.bottom
			: top + boundingRectangle.height

	return (
		pointer.x >= left && pointer.x <= right && pointer.y >= top && pointer.y <= bottom
	)
}

function getNearestFrameWidgetIndex(
	frameState: FrameState,
	manager: DragItem['manager'] | undefined,
	pointer: { x: number; y: number }
): number {
	const droppables = manager?.registry?.droppables
	if (!droppables) return -1

	const frameLen = frameState['frame-widgets'].length
	const frameIdxById = new Map(
		frameState['frame-widgets'].map((entry, idx) => [entry.id, idx] as const)
	)

	let sawFrameContainer = false
	let pointerInsideFrameContainer = false

	let nearestIdx = -1
	let minDistSq = Number.POSITIVE_INFINITY
	let nearestContainingIdx = -1
	let minContainingDistSq = Number.POSITIVE_INFINITY

	for (const droppable of droppables) {
		const droppableGroup = getGroupIdFromDragItem(droppable)
		if (droppableGroup !== 'frame-widgets') continue

		const bounds = droppable.shape?.boundingRectangle
		if (droppable.id === 'frame-widgets') {
			sawFrameContainer = true
			if (isPointInsideBounds(bounds, pointer)) {
				pointerInsideFrameContainer = true
			}
			continue
		}

		const candidateIdxFromData = getIndexFromData(droppable.data)
		const candidateIdx =
			typeof candidateIdxFromData === 'number'
				? candidateIdxFromData
				: typeof droppable?.id === 'string'
					? frameIdxById.get(droppable.id)
					: undefined
		if (candidateIdx === undefined || candidateIdx < 0 || candidateIdx >= frameLen)
			continue

		const center = droppable.shape?.center
		if (typeof center?.x !== 'number' || typeof center?.y !== 'number') continue

		const dx = center.x - pointer.x
		const dy = center.y - pointer.y
		const distSq = dx * dx + dy * dy
		if (distSq < minDistSq) {
			minDistSq = distSq
			nearestIdx = candidateIdx
		}

		if (isPointInsideBounds(bounds, pointer) && distSq < minContainingDistSq) {
			minContainingDistSq = distSq
			nearestContainingIdx = candidateIdx
		}
	}

	if (nearestContainingIdx >= 0) return nearestContainingIdx

	if (sawFrameContainer && !pointerInsideFrameContainer) return -1

	return nearestIdx
}

function getFrameTargetIndex(
	frameState: FrameState,
	target: DragItem | undefined,
	evt?: DragMutationEvent
): number {
	const frameLen = frameState['frame-widgets'].length
	const targetId = target?.id
	if (targetId !== undefined) {
		const explicitIdx = findIndexById(frameState['frame-widgets'], String(targetId))
		if (explicitIdx >= 0) return explicitIdx
	}

	const tgtGroup = getGroupIdFromDragItem(target)

	if (tgtGroup === 'frame-widgets') {
		const targetIdxFromData = getIndexFromData(target?.data)
		if (typeof targetIdxFromData === 'number' && targetIdxFromData >= 0) {
			if (frameLen <= 0) return -1
			return Math.min(targetIdxFromData, frameLen - 1)
		}

		const targetIdx = typeof target?.index === 'number' ? target.index : undefined
		if (typeof targetIdx === 'number' && targetIdx >= 0) {
			if (frameLen <= 0) return -1
			return Math.min(targetIdx, frameLen - 1)
		}
	}

	const op = evt?.operation
	const source = getSource(op)
	const pointer = getPointer(source, getTarget(op))
	if (!pointer || typeof pointer.x !== 'number' || typeof pointer.y !== 'number') {
		return -1
	}

	return getNearestFrameWidgetIndex(frameState, source?.manager, {
		x: pointer.x,
		y: pointer.y,
	})
}

export function findActiveWidget(
	frameState: FrameState,
	sourceId: string
): WidgetView | null {
	return (
		frameState.widgets.find(entry => entry.id === sourceId) ??
		frameState['frame-widgets'].find(entry => entry.id === sourceId) ??
		null
	)
}

export function projectLayoutFrame(
	frameState: FrameState,
	evt: DragMutationEvent,
	frameSlotCount?: number
): FrameState | null {
	const source = getSource(evt.operation)
	if (!source) return null

	const sourceId = String(source.id)
	const hasSource =
		findIndexById(frameState.widgets, sourceId) >= 0 ||
		findIndexById(frameState['frame-widgets'], sourceId) >= 0
	if (!hasSource) return null

	const target = getTarget(evt.operation)

	if (isPoolSourceWithFullFrame(frameState, source, frameSlotCount)) {
		const poolIdx = findIndexById(frameState.widgets, sourceId)
		const frameIdx = getFrameTargetIndex(frameState, target, evt)
		if (frameIdx < 0 || poolIdx < 0) return null

		const srcWidget = frameState.widgets[poolIdx]
		const replacedWidget = frameState['frame-widgets'][frameIdx]
		const nextPool = [
			...frameState.widgets.slice(0, poolIdx),
			...frameState.widgets.slice(poolIdx + 1),
			replacedWidget,
		]
		const nextFrameWidgets = [...frameState['frame-widgets']]
		nextFrameWidgets[frameIdx] = srcWidget

		return {
			...frameState,
			widgets: nextPool,
			'frame-widgets': nextFrameWidgets,
		}
	}

	const moved = move(frameState, evt)
	return moved === frameState ? null : moved
}

export function shouldUseCommittedFrameForOverflowSwap(
	frameState: FrameState,
	evt: DragMutationEvent,
	frameSlotCount: number
): boolean {
	const source = getSource(evt.operation)
	const target = getTarget(evt.operation)
	if (!isPoolSourceWithFullFrame(frameState, source, frameSlotCount)) {
		return false
	}

	return getFrameTargetIndex(frameState, target, evt) >= 0
}

export function shouldCancelOverflowSwapPreview(
	frameState: FrameState,
	evt: DragMutationEvent,
	frameSlotCount: number
): boolean {
	return !shouldUseCommittedFrameForOverflowSwap(frameState, evt, frameSlotCount)
}

export function shouldResetDragPreviewOnOverflowMiss(
	frameState: FrameState,
	evt: DragMutationEvent,
	frameSlotCount: number
): boolean {
	const source = getSource(evt.operation)
	const target = getTarget(evt.operation)
	if (!isPoolSourceWithFullFrame(frameState, source, frameSlotCount)) {
		return false
	}

	return getFrameTargetIndex(frameState, target, evt) < 0
}

function isItemTarget(op: DragOperation | undefined): boolean {
	const targetId = op?.target?.id
	return (
		targetId !== undefined && !isGroupId(typeof targetId === 'string' ? targetId : '')
	)
}

export function getOverlaySize(
	op: DragOperation | undefined
): { width: number; height: number } | null {
	const targetRect = isItemTarget(op) ? op?.target?.shape?.boundingRectangle : undefined
	const dragRect = op?.shape?.current?.boundingRectangle
	const nextRect = targetRect ?? dragRect
	if (!nextRect) return null

	return { width: nextRect.width, height: nextRect.height }
}

export function getDragOverKey(op: DragOperation | undefined): string | null {
	const source = getSource(op)
	const target = getTarget(op)
	if (!source || !target) return null

	const srcGroup = getGroupIdFromDragItem(source) ?? 'unknown'
	const tgtGroup = getGroupIdFromDragItem(target) ?? 'unknown'
	const srcIdxFromData = getIndexFromData(source.data)
	const tgtIdxFromData = getIndexFromData(target.data)
	const srcIdx =
		typeof srcIdxFromData === 'number'
			? srcIdxFromData
			: typeof source.index === 'number'
				? source.index
				: -1
	const tgtIdx =
		typeof tgtIdxFromData === 'number'
			? tgtIdxFromData
			: typeof target.index === 'number'
				? target.index
				: -1

	return `${String(source.id)}:${srcGroup}:${srcIdx}->${String(target.id)}:${tgtGroup}:${tgtIdx}`
}
