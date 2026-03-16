import type { WidgetView } from '@/features/widgets/model/widget-frame'

export interface SortableItemDraft {
	name: string
	clockTimeZone: string
	weatherTemp: string
	weatherIcon: string
	busRoute: string
	busPrediction: string
	clockHours: number | undefined
	clockMinutes: number | undefined
	clockSeconds: number | undefined
	weatherCity: string
	busStop: string
	twitchUser: string
	githubRepo: string
}

const EMPTY_DRAFT: SortableItemDraft = {
	name: '',
	clockTimeZone: '',
	weatherTemp: '',
	weatherIcon: '',
	busRoute: '',
	busPrediction: '',
	clockHours: undefined,
	clockMinutes: undefined,
	clockSeconds: undefined,
	weatherCity: '',
	busStop: '',
	twitchUser: '',
	githubRepo: '',
}

function hasText(value: string): boolean {
	return value.trim().length > 0
}

export function createDraftFromKind(kind: WidgetView['kind']): SortableItemDraft {
	const draft = { ...EMPTY_DRAFT }

	if ('Name' in kind) {
		draft.name = kind.Name.name
	} else if ('Weather' in kind) {
		draft.weatherCity = kind.Weather.city
	} else if ('Bus' in kind) {
		draft.busStop = kind.Bus.stop[0]
	} else if ('Twitch' in kind) {
		draft.twitchUser = kind.Twitch.username
	} else if ('Github' in kind) {
		draft.githubRepo = kind.Github.repo ?? ''
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
		return kind
	}

	if ('Weather' in kind) {
		const city = draft.weatherCity.trim()
		if (!hasText(city)) return null
		const temp = ''
		const humidity = ''
		const wind = ''
		const rain = ''
		const clouds = ''
		const description = ''
		return { Weather: { city, temp, humidity, wind, rain, clouds, description } }
	}

	if ('Bus' in kind) {
		// const stop = draft.busStop.trim()
		// if (!hasText(stop)) return null
		const stop: [string, string] = ['a', 'a']
		const predictions: [string, string, number][] = [['a', 'a', 0]]
		return { Bus: { stop, predictions } }
	}

	if ('Twitch' in kind) {
		const username = draft.twitchUser.trim()
		if (!hasText(username)) return null
		// const live_channels: [string, string, string, string][] = [['', '', '', '']]
		// return { Twitch: { username, live_channels } }
		return {
			Twitch: {
				...kind.Twitch,
				username,
			},
		}
	}
	if ('Github' in kind) {
		console.log('github kind', kind.Github)
		return {
			Github: {
				...kind.Github,
				repo: draft.githubRepo.trim() || null,
			},
		}
	}

	return null
}

export function isKindEditable(kind: WidgetView['kind']): boolean {
	return (
		'Name' in kind ||
		'Clock' in kind ||
		'Weather' in kind ||
		'Bus' in kind ||
		'Twitch' in kind ||
		'Github' in kind
	)
}

export type SortableItemKindVariant =
	| 'Name'
	| 'Clock'
	| 'Weather'
	| 'Bus'
	| 'Twitch'
	| 'Github'
	| 'Readonly'

export function getKindVariant(kind: WidgetView['kind']): SortableItemKindVariant {
	if ('Name' in kind) return 'Name'
	if ('Clock' in kind) return 'Clock'
	if ('Weather' in kind) return 'Weather'
	if ('Bus' in kind) return 'Bus'
	if ('Twitch' in kind) return 'Twitch'
	if ('Github' in kind) return 'Github'
	return 'Readonly'
}

export function isDraftPersistable(
	kind: WidgetView['kind'],
	draft: SortableItemDraft
): boolean {
	if ('Clock' in kind) {
		return true
	}

	return buildKindFromDraft(kind, draft) !== null
}

export function kindSignature(kind: WidgetView['kind']): string {
	return JSON.stringify(kind)
}
