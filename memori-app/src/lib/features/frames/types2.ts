export type LayoutType =
	| 'Full'
	| 'VSplit'
	| 'HSplit'
	| 'VSplitWithRightHSplit'
	| 'HSplitWithTopVSplit'
	| 'VSplitWithLeftHSplit'
	| 'HSplitWithBottomVSplit'
	| 'Fourths'

export type SlotKey =
	| 'Widget'
	| 'Left'
	| 'Right'
	| 'Top'
	| 'Bottom'
	| 'TopLeft'
	| 'TopRight'
	| 'BottomLeft'
	| 'BottomRight'
	| 'LeftTop'
	| 'LeftBottom'
	| 'RightTop'
	| 'RightBottom'

export type WidgetId = string

export type SemanticLayout = {
	type: LayoutType
	assignments: Partial<Record<SlotKey, WidgetId>>
}

export type Rect = {
	x: 0 | 1
	y: 0 | 1
	w: 1 | 2
	h: 1 | 2
}

export type GeometryItem = {
	slot: SlotKey
	rect: Rect
	widgetId?: WidgetId
}

export const TEMPLATE_GEOMETRY: Record<LayoutType, { slot: SlotKey; rect: Rect }[]> = {
	Full: [{ slot: 'Widget', rect: { x: 0, y: 0, w: 2, h: 2 } }],
	VSplit: [
		{ slot: 'Left', rect: { x: 0, y: 0, w: 1, h: 2 } },
		{ slot: 'Right', rect: { x: 1, y: 0, w: 1, h: 2 } },
	],
	HSplit: [
		{ slot: 'Top', rect: { x: 0, y: 0, w: 2, h: 1 } },
		{ slot: 'Bottom', rect: { x: 0, y: 1, w: 2, h: 1 } },
	],
	VSplitWithRightHSplit: [
		{ slot: 'Left', rect: { x: 0, y: 0, w: 1, h: 2 } },
		{ slot: 'RightTop', rect: { x: 1, y: 0, w: 1, h: 1 } },
		{ slot: 'RightBottom', rect: { x: 1, y: 1, w: 1, h: 1 } },
	],
	HSplitWithTopVSplit: [
		{ slot: 'TopLeft', rect: { x: 0, y: 0, w: 1, h: 1 } },
		{ slot: 'TopRight', rect: { x: 1, y: 0, w: 1, h: 1 } },
		{ slot: 'Bottom', rect: { x: 0, y: 1, w: 2, h: 1 } },
	],
	VSplitWithLeftHSplit: [
		{ slot: 'LeftTop', rect: { x: 0, y: 0, w: 1, h: 1 } },
		{ slot: 'LeftBottom', rect: { x: 0, y: 1, w: 1, h: 1 } },
		{ slot: 'Right', rect: { x: 1, y: 0, w: 1, h: 2 } },
	],
	HSplitWithBottomVSplit: [
		{ slot: 'Top', rect: { x: 0, y: 0, w: 2, h: 1 } },
		{ slot: 'BottomLeft', rect: { x: 0, y: 1, w: 1, h: 1 } },
		{ slot: 'BottomRight', rect: { x: 1, y: 1, w: 1, h: 1 } },
	],
	Fourths: [
		{ slot: 'TopLeft', rect: { x: 0, y: 0, w: 1, h: 1 } },
		{ slot: 'TopRight', rect: { x: 1, y: 0, w: 1, h: 1 } },
		{ slot: 'BottomLeft', rect: { x: 0, y: 1, w: 1, h: 1 } },
		{ slot: 'BottomRight', rect: { x: 1, y: 1, w: 1, h: 1 } },
	],
}

export function toGeometry(layout: SemanticLayout): GeometryItem[] {
	return TEMPLATE_GEOMETRY[layout.type].map(({ slot, rect }) => ({
		slot,
		rect,
		widgetId: layout.assignments[slot],
	}))
}

export function toSemantic(type: LayoutType, items: GeometryItem[]): SemanticLayout {
	const expected = new Set(TEMPLATE_GEOMETRY[type].map(entry => entry.slot))
	const assignments: Partial<Record<SlotKey, WidgetId>> = {}

	for (const item of items) {
		if (!expected.has(item.slot)) continue
		if (item.widgetId) assignments[item.slot] = item.widgetId
	}

	return { type, assignments }
}
