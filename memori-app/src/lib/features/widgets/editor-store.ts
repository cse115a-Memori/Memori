import { RuneStore } from '@tauri-store/svelte'
import { LAYOUT_VARIANTS, type LayoutVariant } from '@/features/widgets/model/layout.ts'
import {
	createEmptyWidgetFrame,
	createWidgetFrameEntry,
	createWidgetFrameSignatures,
	DEFAULT_LAYOUT,
	hasSameWidgetIds,
	hasValidWidgetFrameShape,
	initWidgetFrame,
	isLayoutVariant,
	poolChanged,
	type WidgetFrame,
	type WidgetFrameEntry,
	type WidgetFrameSignatures,
} from '@/features/widgets/model/widget-frame.ts'
import type { WidgetsEditorState } from '@/features/widgets/types.ts'
import type { MemoriWidget } from '@/tauri'

export type { WidgetsEditorState } from '@/features/widgets/types.ts'

const initialFrameEntry = createWidgetFrameEntry(
	DEFAULT_LAYOUT,
	createEmptyWidgetFrame()
)

const initialWidgetsEditorState: WidgetsEditorState = {
	widgetPool: [],
	widgetFrames: [initialFrameEntry],
	lastFlashedSignaturesByFrame: [
		createWidgetFrameSignatures(initialFrameEntry.frameLayouts),
	],
}

const widgetsEditorStore = new RuneStore<WidgetsEditorState>(
	'widgets-editor',
	initialWidgetsEditorState,
	{
		autoStart: false,
		saveOnChange: true,
		hooks: {
			error: error => {
				console.error('Widgets editor store error:', error)
			},
		},
	}
)

export const widgetsEditorState = widgetsEditorStore.state

let startPromise: Promise<void> | null = null

function hasValidWidgetFrameSignatures(value: unknown): value is WidgetFrameSignatures {
	if (!value || typeof value !== 'object' || Array.isArray(value)) return false
	return LAYOUT_VARIANTS.every(
		layout => typeof (value as Record<string, unknown>)[layout] === 'string'
	)
}

function isWidgetFrameEntry(value: unknown): value is WidgetFrameEntry {
	if (!value || typeof value !== 'object' || Array.isArray(value)) return false
	const candidate = value as { activeLayout?: unknown; frameLayouts?: unknown }
	return (
		isLayoutVariant(candidate.activeLayout) &&
		hasValidWidgetFrameShape(candidate.frameLayouts as WidgetFrame | undefined)
	)
}

function normalizeWidgetFrameEntry(value: unknown): WidgetFrameEntry {
	if (isWidgetFrameEntry(value)) return value
	return createWidgetFrameEntry()
}

function frameNeedsReinit(frame: WidgetFrame, data: MemoriWidget[]): boolean {
	return LAYOUT_VARIANTS.some(layout => {
		const layoutFrame = frame[layout]
		if (!layoutFrame) return true

		const assignedIds = [...layoutFrame.widgets, ...layoutFrame['frame-widgets']].map(
			widget => widget.widgetId
		)

		return !hasSameWidgetIds(
			assignedIds,
			data.map(widget => widget.id)
		)
	})
}

function normalizeWidgetFramesState(): void {
	const rawFrames = Array.isArray(widgetsEditorState.widgetFrames as unknown)
		? (widgetsEditorState.widgetFrames as unknown[])
		: []
	const normalized = rawFrames.map(normalizeWidgetFrameEntry)

	if (normalized.length === 0) normalized.push(createWidgetFrameEntry())
	widgetsEditorState.widgetFrames = normalized
}

function normalizeFlashedSignaturesByFrameState(): void {
	const rawByFrame = Array.isArray(
		widgetsEditorState.lastFlashedSignaturesByFrame as unknown
	)
		? (widgetsEditorState.lastFlashedSignaturesByFrame as unknown[])
		: []

	widgetsEditorState.lastFlashedSignaturesByFrame = widgetsEditorState.widgetFrames.map(
		(entry, frameIdx) => {
			const fromArray = rawByFrame[frameIdx]
			if (hasValidWidgetFrameSignatures(fromArray)) return fromArray
			return createWidgetFrameSignatures(entry.frameLayouts)
		}
	)
}

function replaceWidgetFrameEntry(
	frameIdx: number,
	entry: WidgetFrameEntry
): WidgetFrameEntry {
	const nextFrames = [...widgetsEditorState.widgetFrames]
	while (nextFrames.length <= frameIdx) {
		nextFrames.push(createWidgetFrameEntry())
	}

	nextFrames[frameIdx] = entry
	widgetsEditorState.widgetFrames = nextFrames
	return entry
}

export function ensureWidgetFrameEntry(frameIdx: number): WidgetFrameEntry {
	const existing = widgetsEditorState.widgetFrames[frameIdx] as unknown
	if (isWidgetFrameEntry(existing)) return existing

	const normalized = normalizeWidgetFrameEntry(existing)
	return replaceWidgetFrameEntry(frameIdx, normalized)
}

export function setWidgetFrameLayout(frameIdx: number, layout: LayoutVariant): void {
	const frameEntry = ensureWidgetFrameEntry(frameIdx)
	if (frameEntry.activeLayout === layout) return
	replaceWidgetFrameEntry(
		frameIdx,
		createWidgetFrameEntry(layout, frameEntry.frameLayouts)
	)
}

export function setWidgetFrame(frameIdx: number, frame: WidgetFrame): void {
	const frameEntry = ensureWidgetFrameEntry(frameIdx)
	replaceWidgetFrameEntry(
		frameIdx,
		createWidgetFrameEntry(frameEntry.activeLayout, frame)
	)
}

export function syncWidgetPoolAndFrame(
	frameIdx: number,
	data: MemoriWidget[]
): boolean {
	ensureWidgetFrameEntry(frameIdx)

	const frameEntries = widgetsEditorState.widgetFrames.map((_, index) =>
		ensureWidgetFrameEntry(index)
	)
	const needsInit =
		poolChanged(widgetsEditorState.widgetPool, data) ||
		frameEntries.some(entry => frameNeedsReinit(entry.frameLayouts, data))

	widgetsEditorState.widgetPool = data
	if (!needsInit) return false

	const nextFrames = frameEntries.map(entry =>
		createWidgetFrameEntry(entry.activeLayout, initWidgetFrame(data))
	)
	widgetsEditorState.widgetFrames = nextFrames
	widgetsEditorState.lastFlashedSignaturesByFrame = nextFrames.map(entry =>
		createWidgetFrameSignatures(entry.frameLayouts)
	)
	return true
}

export function startWidgetsEditorStore(): Promise<void> {
	startPromise ??= widgetsEditorStore
		.start()
		.then(() => {
			normalizeWidgetFramesState()
			normalizeFlashedSignaturesByFrameState()
		})
		.catch(error => {
			startPromise = null
			throw error
		})

	return startPromise
}
