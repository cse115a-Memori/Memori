import { RuneStore } from '@tauri-store/svelte'
import {
	getLayoutSlotCount,
	LAYOUT_VARIANTS,
	type LayoutVariant,
} from '@/features/widgets/model/layout'
import {
	DEFAULT_LAYOUT,
	isLayoutVariant,
	type WidgetFrame,
	type WidgetFrameEntry,
	type WidgetView,
} from '@/features/widgets/model/widget-frame'
import type { FrameDraft, WidgetsState } from '@/features/widgets/types'
import type { MemoriWidget } from '@/tauri'

export type { FrameDraft, WidgetsState } from '@/features/widgets/types'

const DEFAULT_FRAME_TIME_SECONDS = 5

const initialWidgetsState: WidgetsState = {
	activeFrameIdx: 0,
	frameTime: DEFAULT_FRAME_TIME_SECONDS,
	widgets: [],
	frames: [],
}

const widgetsStore = new RuneStore<WidgetsState>('widgets', initialWidgetsState, {
	autoStart: false,
	saveOnChange: true,
	hooks: {
		error: error => {
			console.error('Widgets store error:', error)
		},
	},
})

export const widgetsState = widgetsStore.state

let startPromise: Promise<void> | null = null

export function resetWidgets(): void {
	widgetsState.activeFrameIdx = initialWidgetsState.activeFrameIdx
	widgetsState.frameTime = initialWidgetsState.frameTime
	widgetsState.widgets = initialWidgetsState.widgets
	widgetsState.frames = initialWidgetsState.frames
}

function isMemoriWidget(value: unknown): value is MemoriWidget {
	if (!value || typeof value !== 'object' || Array.isArray(value)) return false
	const candidate = value as { id?: unknown; kind?: unknown }
	return typeof candidate.id === 'number' && candidate.kind !== undefined
}

function dedupeWidgetsById(widgets: MemoriWidget[]): MemoriWidget[] {
	const seen = new Set<Memori.WidgetId>()
	const deduped: MemoriWidget[] = []

	for (const widget of widgets) {
		if (seen.has(widget.id)) continue
		seen.add(widget.id)
		deduped.push(widget)
	}

	return deduped
}

function createWidgetMap(widgets: MemoriWidget[]): Map<Memori.WidgetId, MemoriWidget> {
	return new Map(widgets.map(widget => [widget.id, widget] as const))
}

function normalizeWidgetStateShape(): void {
	const candidate = widgetsState as unknown as {
		widgets?: unknown
		widgetsById?: unknown
		widgetOrder?: unknown
	}

	if (Array.isArray(candidate.widgets)) {
		const normalized = dedupeWidgetsById(candidate.widgets.filter(isMemoriWidget))
		widgetsState.widgets = normalized
		return
	}

	const legacyById =
		candidate.widgetsById &&
		typeof candidate.widgetsById === 'object' &&
		!Array.isArray(candidate.widgetsById)
			? (candidate.widgetsById as Record<string, unknown>)
			: null
	if (!legacyById) {
		widgetsState.widgets = []
		return
	}

	const parsedById = {} as Record<Memori.WidgetId, MemoriWidget>
	for (const value of Object.values(legacyById)) {
		if (!isMemoriWidget(value)) continue
		parsedById[value.id] = value
	}

	const legacyOrder = Array.isArray(candidate.widgetOrder)
		? candidate.widgetOrder.filter(
				(id): id is Memori.WidgetId =>
					typeof id === 'number' && parsedById[id] !== undefined
			)
		: []

	const orderedWidgets =
		legacyOrder.length > 0
			? legacyOrder.map(id => parsedById[id] as MemoriWidget)
			: Object.values(parsedById)

	widgetsState.widgets = dedupeWidgetsById(orderedWidgets)
}

function toNonNegativeIndex(value: number): number {
	if (!Number.isFinite(value)) return 0
	return Math.max(0, Math.trunc(value))
}

function clampIndex(value: number, upperBound: number): number {
	if (upperBound <= 0) return 0
	return Math.min(Math.max(value, 0), upperBound - 1)
}

function normalizeLayoutAssignment(
	rawAssignment: unknown,
	widgets: MemoriWidget[],
	widgetById: Map<Memori.WidgetId, MemoriWidget>,
	slotCount: number,
	fillMissing: boolean
): MemoriWidget[] {
	const normalized: MemoriWidget[] = []
	const seen = new Set<Memori.WidgetId>()

	if (Array.isArray(rawAssignment)) {
		for (const rawEntry of rawAssignment) {
			const widgetId =
				typeof rawEntry === 'number'
					? rawEntry
					: isMemoriWidget(rawEntry)
						? rawEntry.id
						: undefined
			if (widgetId === undefined || seen.has(widgetId)) continue

			const widget = widgetById.get(widgetId)
			if (!widget) continue

			normalized.push(widget)
			seen.add(widgetId)
			if (normalized.length >= slotCount) return normalized
		}
	}

	if (fillMissing && normalized.length < slotCount) {
		for (const widget of widgets) {
			if (seen.has(widget.id)) continue
			normalized.push(widget)
			seen.add(widget.id)
			if (normalized.length >= slotCount) break
		}
	}

	return normalized
}

function extractLayoutAssignment(
	candidate: Record<string, unknown>,
	layout: LayoutVariant
): unknown {
	const rawAssignments = candidate.layoutAssignments
	if (
		rawAssignments &&
		typeof rawAssignments === 'object' &&
		!Array.isArray(rawAssignments)
	) {
		return (rawAssignments as Record<string, unknown>)[layout]
	}

	const rawLegacyLayouts = candidate.frameLayouts
	if (
		rawLegacyLayouts &&
		typeof rawLegacyLayouts === 'object' &&
		!Array.isArray(rawLegacyLayouts)
	) {
		const layoutState = (rawLegacyLayouts as Record<string, unknown>)[layout]
		if (!layoutState || typeof layoutState !== 'object' || Array.isArray(layoutState)) {
			return undefined
		}

		const frameWidgets = (layoutState as Record<string, unknown>)['frame-widgets']
		if (!Array.isArray(frameWidgets)) return undefined

		return frameWidgets
			.map(entry => {
				if (!entry || typeof entry !== 'object' || Array.isArray(entry))
					return undefined
				const rawWidgetId = (entry as { widgetId?: unknown }).widgetId
				return typeof rawWidgetId === 'number' ? rawWidgetId : undefined
			})
			.filter((widgetId): widgetId is Memori.WidgetId => widgetId !== undefined)
	}

	return undefined
}

function createFrameDraft(widgets: MemoriWidget[]): FrameDraft {
	const widgetById = createWidgetMap(widgets)
	const layoutAssignments = LAYOUT_VARIANTS.reduce(
		(acc, layout) => {
			acc[layout] = normalizeLayoutAssignment(
				[],
				widgets,
				widgetById,
				getLayoutSlotCount(layout),
				true
			)
			return acc
		},
		{} as Record<LayoutVariant, MemoriWidget[]>
	)

	return {
		activeLayout: DEFAULT_LAYOUT,
		layoutAssignments,
	}
}

function normalizeFrameDraft(
	rawFrame: unknown,
	widgets: MemoriWidget[],
	widgetById: Map<Memori.WidgetId, MemoriWidget>,
	fillMissing: boolean
): FrameDraft {
	if (!rawFrame || typeof rawFrame !== 'object' || Array.isArray(rawFrame)) {
		return createFrameDraft(widgets)
	}

	const candidate = rawFrame as Record<string, unknown>
	const activeLayout = isLayoutVariant(candidate.activeLayout)
		? candidate.activeLayout
		: DEFAULT_LAYOUT

	const layoutAssignments = LAYOUT_VARIANTS.reduce(
		(acc, layout) => {
			const rawAssignment = extractLayoutAssignment(candidate, layout)
			acc[layout] = normalizeLayoutAssignment(
				rawAssignment,
				widgets,
				widgetById,
				getLayoutSlotCount(layout),
				fillMissing
			)
			return acc
		},
		{} as Record<LayoutVariant, MemoriWidget[]>
	)

	return { activeLayout, layoutAssignments }
}

function normalizeFramesState(fillMissing: boolean): void {
	const widgets = widgetsState.widgets
	const widgetById = createWidgetMap(widgets)
	const rawFrames = Array.isArray(widgetsState.frames as unknown)
		? (widgetsState.frames as unknown[])
		: []

	const nextFrames = rawFrames.map(rawFrame =>
		normalizeFrameDraft(rawFrame, widgets, widgetById, fillMissing)
	)
	if (nextFrames.length === 0) {
		nextFrames.push(createFrameDraft(widgets))
	}

	widgetsState.frames = nextFrames
	widgetsState.activeFrameIdx = clampIndex(
		toNonNegativeIndex(widgetsState.activeFrameIdx),
		nextFrames.length
	)
}

function toWidgetView(
	widget: MemoriWidget,
	group: 'pool' | 'frame',
	layout: LayoutVariant
): WidgetView {
	return {
		id: `${group}-${layout}-${widget.id}`,
		widgetId: widget.id,
		kind: widget.kind,
	}
}

function toWidgetFrame(frameDraft: FrameDraft): WidgetFrame {
	const widgetById = createWidgetMap(widgetsState.widgets)

	return LAYOUT_VARIANTS.reduce((acc, layout) => {
		const assignedWidgets = (frameDraft.layoutAssignments[layout] ?? []).flatMap(
			widget => {
				const canonicalWidget = widgetById.get(widget.id)
				if (!canonicalWidget) return []
				return [canonicalWidget]
			}
		)
		const assignedIds = new Set(assignedWidgets.map(widget => widget.id))

		const frameWidgets = assignedWidgets.map(widget =>
			toWidgetView(widget, 'frame', layout)
		)

		const poolWidgets = widgetsState.widgets
			.filter(widget => !assignedIds.has(widget.id))
			.map(widget => toWidgetView(widget, 'pool', layout))

		acc[layout] = {
			widgets: poolWidgets,
			'frame-widgets': frameWidgets,
		}
		return acc
	}, {} as WidgetFrame)
}

function replaceFrame(frameIdx: number, frame: FrameDraft): void {
	const nextFrames = [...widgetsState.frames]
	nextFrames[frameIdx] = frame
	widgetsState.frames = nextFrames
}

export function normalizeWidgetsState(widgetData?: MemoriWidget[]): void {
	if (widgetData) {
		const widgetsById = createWidgetMap(widgetsState.widgets)
		widgetsState.widgets = dedupeWidgetsById(
				widgetData.map(widget => {
					const stored = widgetsById.get(widget.id)
					if (!stored) return widget
					// Always use fresh data for Github
					if ('Clock' in widget.kind) return widget
					if ('Github' in widget.kind) return widget
        if ('Twitch' in widget.kind) return widget
					if ('Bus' in widget.kind) return widget
				if ('Weather' in widget.kind) return widget
				return stored
			})
		)
	} else {
		normalizeWidgetStateShape()
	}

	normalizeFramesState(true)
}

export function ensureFrameDraft(frameIdx: number): FrameDraft {
	const safeIdx = toNonNegativeIndex(frameIdx)
	const nextFrames = [...widgetsState.frames]
	let didChange = false

	if (nextFrames.length === 0) {
		nextFrames.push(createFrameDraft(widgetsState.widgets))
		didChange = true
	}

	if (safeIdx < nextFrames.length) {
		if (didChange) widgetsState.frames = nextFrames
		return nextFrames[safeIdx] as FrameDraft
	}

	while (nextFrames.length <= safeIdx) {
		nextFrames.push(createFrameDraft(widgetsState.widgets))
		didChange = true
	}

	if (didChange) widgetsState.frames = nextFrames
	return nextFrames[safeIdx] as FrameDraft
}

export function selectWidgetFrameEntry(frameIdx: number): WidgetFrameEntry {
	const safeIdx = toNonNegativeIndex(frameIdx)
	const frameDraft =
		widgetsState.frames[safeIdx] ?? createFrameDraft(widgetsState.widgets)
	return {
		activeLayout: frameDraft.activeLayout,
		frameLayouts: toWidgetFrame(frameDraft),
	}
}

export function setActiveFrameIdx(frameIdx: number): void {
	widgetsState.activeFrameIdx = clampIndex(
		toNonNegativeIndex(frameIdx),
		widgetsState.frames.length
	)
}

export function setWidgetFrameLayout(frameIdx: number, layout: LayoutVariant): void {
	const safeIdx = toNonNegativeIndex(frameIdx)
	const frameDraft = ensureFrameDraft(safeIdx)
	if (frameDraft.activeLayout === layout) return

	replaceFrame(safeIdx, {
		...frameDraft,
		activeLayout: layout,
	})
}

export function setWidgetFrame(frameIdx: number, frame: WidgetFrame): void {
	const safeIdx = toNonNegativeIndex(frameIdx)
	const frameDraft = ensureFrameDraft(safeIdx)
	const activeLayout = frameDraft.activeLayout
	const widgetById = createWidgetMap(widgetsState.widgets)
	const nextFrameWidgets = frame[activeLayout]['frame-widgets'].map(
		widget => widget.widgetId
	)
	const nextAssignments = normalizeLayoutAssignment(
		nextFrameWidgets,
		widgetsState.widgets,
		widgetById,
		getLayoutSlotCount(activeLayout),
		false
	)

	replaceFrame(safeIdx, {
		...frameDraft,
		layoutAssignments: {
			...frameDraft.layoutAssignments,
			[activeLayout]: nextAssignments,
		},
	})
}

export function updateWidgetKind(
	widgetId: Memori.WidgetId,
	kind: MemoriWidget['kind']
): boolean {
	const widgetIndex = widgetsState.widgets.findIndex(widget => widget.id === widgetId)
	if (widgetIndex < 0) return false

	const currentWidget = widgetsState.widgets[widgetIndex] as MemoriWidget
	const nextWidget = {
		...currentWidget,
		kind,
	}

	const nextWidgets = [...widgetsState.widgets]
	nextWidgets[widgetIndex] = nextWidget
	widgetsState.widgets = nextWidgets

	let frameChanged = false
	const nextFrames = widgetsState.frames.map(frameDraft => {
		let assignmentChanged = false
		const nextLayoutAssignments = LAYOUT_VARIANTS.reduce(
			(acc, layout) => {
				const nextAssignment = frameDraft.layoutAssignments[layout].map(widget => {
					if (widget.id !== widgetId) return widget
					assignmentChanged = true
					return nextWidget
				})
				acc[layout] = nextAssignment
				return acc
			},
			{} as Record<LayoutVariant, MemoriWidget[]>
		)

		if (!assignmentChanged) return frameDraft
		frameChanged = true
		return {
			...frameDraft,
			layoutAssignments: nextLayoutAssignments,
		}
	})

	if (frameChanged) {
		widgetsState.frames = nextFrames
	}

	return true
}

export function syncWidgets(widgetData: MemoriWidget[]): void {
	normalizeWidgetsState(widgetData)
}

export function startWidgetsStore(): Promise<void> {
	startPromise ??= widgetsStore
		.start()
		.then(() => {
			normalizeWidgetsState()
		})
		.catch(error => {
			startPromise = null
			throw error
		})

	return startPromise
}
