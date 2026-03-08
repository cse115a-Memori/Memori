import type { WidgetView } from '@/features/widgets/model/widget-frame'
import { isClockTimezoneDraftValue } from './widget-clock'

export interface SortableItemDraft {
	name: string
	clockTimeZone: string
	weatherTemp: string
	weatherIcon: string
	busRoute: string
	busPrediction: string
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
		draft.weatherTemp = kind.Weather.temp
		draft.weatherIcon = kind.Weather.icon
	} else if ('Bus' in kind) {
		draft.busRoute = kind.Bus.route
		draft.busPrediction = kind.Bus.prediction
	} else if ('Twitch' in kind) {
		draft.twitchUser = kind.Twitch.user
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
	if ('Github' in kind) {
    return {
      Github: {
        ...kind.Github,
        repo: draft.githubRepo.trim() || null,
      }
    }
  }

	return null
}

export function isKindEditable(kind: WidgetView['kind']): boolean {
  return 'Name' in kind || 'Clock' in kind || 'Weather' in kind ||
    'Bus' in kind || 'Twitch' in kind || 'Github' in kind
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
	if ('Clock' in kind) {
		return isClockTimezoneDraftValue(draft.clockTimeZone)
	}

	return buildKindFromDraft(kind, draft) !== null
}

export function kindSignature(kind: WidgetView['kind']): string {
	return JSON.stringify(kind)
}
