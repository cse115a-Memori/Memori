import type { WidgetView } from '@/features/widgets/model/widget-frame'

export interface SortableItemDraft {
	name: string
	clockHours: number | undefined
	clockMinutes: number | undefined
	clockSeconds: number | undefined
	weatherTemp: string
	weatherIcon: string
	busRoute: string
	busPrediction: string
	twitchUser: string
}

const EMPTY_DRAFT: SortableItemDraft = {
	name: '',
	clockHours: undefined,
	clockMinutes: undefined,
	clockSeconds: undefined,
	weatherTemp: '',
	weatherIcon: '',
	busRoute: '',
	busPrediction: '',
	twitchUser: '',
}

function hasText(value: string): boolean {
	return value.trim().length > 0
}

function clampUnit(value: number | undefined, max: number, fallback: number): number {
	if (value === undefined || Number.isNaN(value)) return fallback
	const truncated = Math.trunc(value)
	return Math.min(max, Math.max(0, truncated))
}

export function createDraftFromKind(kind: WidgetView['kind']): SortableItemDraft {
	const draft = { ...EMPTY_DRAFT }

	if ('Name' in kind) {
		draft.name = kind.Name.name
	} else if ('Clock' in kind) {
		draft.clockHours = kind.Clock.hours
		draft.clockMinutes = kind.Clock.minutes
		draft.clockSeconds = kind.Clock.seconds
	} else if ('Weather' in kind) {
		draft.weatherTemp = kind.Weather.temp
		draft.weatherIcon = kind.Weather.icon
	} else if ('Bus' in kind) {
		draft.busRoute = kind.Bus.route
		draft.busPrediction = kind.Bus.prediction
	} else if ('Twitch' in kind) {
		draft.twitchUser = kind.Twitch.user
	}

	return draft
}

export function buildKindFromDraft(
	kind: WidgetView['kind'],
	draft: SortableItemDraft
): WidgetView['kind'] | null {
	if ('Name' in kind) {
		const name = draft.name.trim()
		if (!hasText(name)) return null
		return { Name: { name } }
	}

	if ('Clock' in kind) {
		return {
			Clock: {
				hours: clampUnit(draft.clockHours, 23, kind.Clock.hours),
				minutes: clampUnit(draft.clockMinutes, 59, kind.Clock.minutes),
				seconds: clampUnit(draft.clockSeconds, 59, kind.Clock.seconds),
			},
		}
	}

	if ('Weather' in kind) {
		const temp = draft.weatherTemp.trim()
		const icon = draft.weatherIcon.trim()
		if (!hasText(temp) || !hasText(icon)) return null
		return { Weather: { temp, icon } }
	}

	if ('Bus' in kind) {
		const route = draft.busRoute.trim()
		const prediction = draft.busPrediction.trim()
		if (!hasText(route) || !hasText(prediction)) return null
		return { Bus: { route, prediction } }
	}

	if ('Twitch' in kind) {
		const user = draft.twitchUser.trim()
		if (!hasText(user)) return null
		return { Twitch: { user } }
	}

	return null
}

export function isKindEditable(kind: WidgetView['kind']): boolean {
	return !('Github' in kind)
}

export type SortableItemKindVariant =
	| 'Name'
	| 'Clock'
	| 'Weather'
	| 'Bus'
	| 'Twitch'
	| 'Readonly'

export function getKindVariant(kind: WidgetView['kind']): SortableItemKindVariant {
	if ('Name' in kind) return 'Name'
	if ('Clock' in kind) return 'Clock'
	if ('Weather' in kind) return 'Weather'
	if ('Bus' in kind) return 'Bus'
	if ('Twitch' in kind) return 'Twitch'
	return 'Readonly'
}

export function isDraftPersistable(
	kind: WidgetView['kind'],
	draft: SortableItemDraft
): boolean {
	return buildKindFromDraft(kind, draft) !== null
}

export function kindSignature(kind: WidgetView['kind']): string {
	return JSON.stringify(kind)
}
