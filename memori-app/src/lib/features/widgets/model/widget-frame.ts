import {
	getLayoutSlotCount,
	LAYOUT_VARIANTS,
	type LayoutVariant,
} from '@/features/widgets/model/layout'
import type { MemoriWidget } from '@/tauri'
import {
	type Frame,
	type FrameHash,
	GROUP_IDS,
	type GroupId,
	type LayoutsInFrame,
	type WidgetDisplay,
	type WidgetView,
} from './widget-frame.types'

export const DEFAULT_LAYOUT: LayoutVariant = 'Full'
export type {
	Frame as WidgetFrameEntry,
	FrameHash as WidgetFrameSignatures,
	GroupId,
	GroupWidgets as FrameState,
	LayoutsInFrame as WidgetFrame,
	WidgetDisplay,
	WidgetView,
} from './widget-frame.types'
export { GROUP_IDS } from './widget-frame.types'

function createIdCountMap(ids: Memori.WidgetId[]): Map<Memori.WidgetId, number> {
	const counts = new Map<Memori.WidgetId, number>()
	for (const id of ids) {
		counts.set(id, (counts.get(id) ?? 0) + 1)
	}
	return counts
}

export function hasSameWidgetIds(
	left: Memori.WidgetId[],
	right: Memori.WidgetId[]
): boolean {
	if (left.length !== right.length) return false

	const leftCounts = createIdCountMap(left)
	const rightCounts = createIdCountMap(right)
	if (leftCounts.size !== rightCounts.size) return false

	for (const [id, count] of leftCounts) {
		if (rightCounts.get(id) !== count) return false
	}

	return true
}

export function isGroupId(value: unknown): value is GroupId {
	return typeof value === 'string' && (GROUP_IDS as readonly string[]).includes(value)
}

export function isLayoutVariant(value: unknown): value is LayoutVariant {
	return (
		typeof value === 'string' && (LAYOUT_VARIANTS as readonly string[]).includes(value)
	)
}

export function hasValidWidgetFrameShape(
	widgetFrame: LayoutsInFrame | undefined
): widgetFrame is LayoutsInFrame {
	if (!widgetFrame) return false
	return LAYOUT_VARIANTS.every(variant => {
		const frame = widgetFrame[variant]
		return (
			!!frame && Array.isArray(frame.widgets) && Array.isArray(frame['frame-widgets'])
		)
	})
}

export function createEmptyWidgetFrame(): LayoutsInFrame {
	return LAYOUT_VARIANTS.reduce((acc, variant) => {
		acc[variant] = { widgets: [], 'frame-widgets': [] }
		return acc
	}, {} as LayoutsInFrame)
}

export function createWidgetFrameEntry(
	layout: LayoutVariant = DEFAULT_LAYOUT,
	frame: LayoutsInFrame = createEmptyWidgetFrame()
): Frame {
	return { activeLayout: layout, frameLayouts: frame }
}

export function initWidgetFrame(widgets: MemoriWidget[]): LayoutsInFrame {
	return LAYOUT_VARIANTS.reduce((acc, variant) => {
		acc[variant] = {
			widgets: widgets.map(
				(widget, index): WidgetView => ({
					id: `pool-${index}`,
					widgetId: widget.id,
					kind: widget.kind,
				})
			),
			'frame-widgets': [],
		}
		return acc
	}, {} as LayoutsInFrame)
}

export function poolChanged(pool: MemoriWidget[], widgetData: MemoriWidget[]): boolean {
	return !hasSameWidgetIds(
		pool.map(widget => widget.id),
		widgetData.map(widget => widget.id)
	)
}

export function createWidgetLayoutSignature(
	widgetFrame: LayoutsInFrame,
	layout: LayoutVariant
): string {
	const frame = widgetFrame[layout]
	const widgets = frame?.widgets.map(widget => String(widget.widgetId)).join(',') ?? ''
	const frameWidgets =
		frame?.['frame-widgets'].map(widget => String(widget.widgetId)).join(',') ?? ''
	return `${widgets}|${frameWidgets}`
}

export function getFrameWidgetCount(
	widgetFrame: LayoutsInFrame,
	layout: LayoutVariant
): number {
	return widgetFrame[layout]['frame-widgets'].length
}

export function createWidgetFrameSignatures(widgetFrame: LayoutsInFrame): FrameHash {
	return LAYOUT_VARIANTS.reduce((acc, layout) => {
		acc[layout] = createWidgetLayoutSignature(widgetFrame, layout)
		return acc
	}, {} as FrameHash)
}

export function kindToDisplay(kind: MemoriWidget['kind']): WidgetDisplay {
	if ('Name' in kind) {
		return { name: 'Name', content: kind.Name.name }
	}

	if ('Clock' in kind) {
		const hours = String(kind.Clock.hours).padStart(2, '0')
		const minutes = String(kind.Clock.minutes).padStart(2, '0')
		const seconds = String(kind.Clock.seconds).padStart(2, '0')
		return { name: 'Clock', content: `${hours}:${minutes}:${seconds}` }
	}

	if ('Weather' in kind) {
		return { name: 'Weather', content: `${kind.Weather.temp}°C` }
	}

	if ('Bus' in kind) {
		return { name: `Bus ${kind.Bus.route}`, content: kind.Bus.prediction }
	}

	if ('Github' in kind) {
		const repo = kind.Github.repo ? `/${kind.Github.repo}` : ''
		return { name: 'GitHub', content: `${kind.Github.username}${repo}` }
	}

	if ('Twitch' in kind) {
		return { name: 'Twitch', content: kind.Twitch.user }
	}

	return { name: 'Widget', content: '' }
}

export function isLayoutFilled(
	widgetFrame: LayoutsInFrame,
	layout: LayoutVariant
): boolean {
	return getFrameWidgetCount(widgetFrame, layout) === getLayoutSlotCount(layout)
}
