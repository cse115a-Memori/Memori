import { RuneStore } from '@tauri-store/svelte'
import { LAYOUT_VARIANTS, type LayoutVariant } from '@/model/layout.ts'
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
} from '@/model/widget-frame.ts'
import { getSystemOptions } from '@/services/system-service'
import type { AppState } from '@/stores/app/types.ts'
import type { MemoriWidget } from '@/tauri'

export type { AppState, LocationStatus } from '@/stores/app/types.ts'

const initialFrameEntry = createWidgetFrameEntry(
	DEFAULT_LAYOUT,
	createEmptyWidgetFrame()
)

const initialAppState: AppState = {
	locationStatus: 'prompt',
	lastKnownLocation: null,
	onboarded: false,
	lastKnownDeviceId: null,
	systemOptions: getSystemOptions(),
	widgetPool: [],
	widgetFrames: [initialFrameEntry],
	lastFlashedSignaturesByFrame: [
		createWidgetFrameSignatures(initialFrameEntry.frameLayouts),
	],
}

const appStore = new RuneStore<AppState>('app-store', initialAppState, {
	autoStart: false,
	saveOnChange: true,
	hooks: {
		error: error => {
			console.error('App store error:', error)
		},
	},
})

export const appState = appStore.state

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

	if (Array.isArray(value)) {
		const layout = isLayoutVariant(value[0]) ? value[0] : DEFAULT_LAYOUT
		const frame = hasValidWidgetFrameShape(value[1] as WidgetFrame | undefined)
			? (value[1] as WidgetFrame)
			: createEmptyWidgetFrame()
		return createWidgetFrameEntry(layout, frame)
	}

	if (value && typeof value === 'object') {
		const candidate = value as {
			activeLayout?: unknown
			layout?: unknown
			frameLayouts?: unknown
			frame?: unknown
		}
		const layout = isLayoutVariant(candidate.activeLayout)
			? candidate.activeLayout
			: isLayoutVariant(candidate.layout)
				? candidate.layout
				: DEFAULT_LAYOUT
		const frame = hasValidWidgetFrameShape(
			candidate.frameLayouts as WidgetFrame | undefined
		)
			? (candidate.frameLayouts as WidgetFrame)
			: hasValidWidgetFrameShape(candidate.frame as WidgetFrame | undefined)
				? (candidate.frame as WidgetFrame)
				: createEmptyWidgetFrame()
		return createWidgetFrameEntry(layout, frame)
	}

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
	const rawFrames = Array.isArray(appState.widgetFrames as unknown)
		? (appState.widgetFrames as unknown[])
		: []
	const normalized = rawFrames.map(normalizeWidgetFrameEntry)

	if (normalized.length === 0) normalized.push(createWidgetFrameEntry())
	appState.widgetFrames = normalized
}

function normalizeFlashedSignaturesByFrameState(): void {
	const rawByFrame = Array.isArray(appState.lastFlashedSignaturesByFrame as unknown)
		? (appState.lastFlashedSignaturesByFrame as unknown[])
		: []

	appState.lastFlashedSignaturesByFrame = appState.widgetFrames.map(
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
	const nextFrames = [...appState.widgetFrames]
	while (nextFrames.length <= frameIdx) {
		nextFrames.push(createWidgetFrameEntry())
	}

	nextFrames[frameIdx] = entry
	appState.widgetFrames = nextFrames
	return entry
}

export function ensureWidgetFrameEntry(frameIdx: number): WidgetFrameEntry {
	const existing = appState.widgetFrames[frameIdx] as unknown
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

	const frameEntries = appState.widgetFrames.map((_, index) =>
		ensureWidgetFrameEntry(index)
	)
	const needsInit =
		poolChanged(appState.widgetPool, data) ||
		frameEntries.some(entry => frameNeedsReinit(entry.frameLayouts, data))

	appState.widgetPool = data
	if (!needsInit) return false

	const nextFrames = frameEntries.map(entry =>
		createWidgetFrameEntry(entry.activeLayout, initWidgetFrame(data))
	)
	appState.widgetFrames = nextFrames
	appState.lastFlashedSignaturesByFrame = nextFrames.map(entry =>
		createWidgetFrameSignatures(entry.frameLayouts)
	)
	return true
}

export function startAppStore(): Promise<void> {
	startPromise ??= appStore
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
