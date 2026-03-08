export type CompactClock = { time: string; zone: string }

export const CLOCK_TIMEZONE_OPTIONS = ['UTC', 'America/New_York', 'Europe/London']

export function getCurrentSystemTimeZone(): string | null {
	return Intl.DateTimeFormat().resolvedOptions().timeZone ?? null
}

export function isValidClockTimeZone(timeZone: string): boolean {
	try {
		new Intl.DateTimeFormat(undefined, { timeZone }).format(new Date())
		return true
	} catch {
		return false
	}
}

export function isClockTimezoneDraftValue(value: string): boolean {
	return isValidClockTimeZone(value)
}

export function toClockTimezoneDraftValue(
	timeZone: string | null | undefined,
	systemTimeZone = getCurrentSystemTimeZone(),
): string {
	if (timeZone && isClockTimezoneDraftValue(timeZone)) {
		return timeZone
	}

	return systemTimeZone ?? 'UTC'
}

export function resolveClockTimeZone(value: string): string | undefined {
	return isValidClockTimeZone(value) ? value : undefined
}

export function getClockTimezoneLabel(value: string): string {
	return value
}

export function formatCompactClock(date: Date, timeZone?: string): CompactClock {
	try {
		const parts = new Intl.DateTimeFormat(undefined, {
			hour: 'numeric',
			minute: '2-digit',
			hour12: true,
			timeZone,
			timeZoneName: 'short',
		}).formatToParts(date)
		const hour = parts.find((part) => part.type === 'hour')?.value ?? ''
		const minute = parts.find((part) => part.type === 'minute')?.value ?? '00'
		const dayPeriod = parts.find((part) => part.type === 'dayPeriod')?.value?.toUpperCase() ?? ''
		const zone = parts.find((part) => part.type === 'timeZoneName')?.value ?? 'Local'

		return {
			time: `${hour}:${minute}${dayPeriod ? ` ${dayPeriod}` : ''}`,
			zone,
		}
	} catch {
		const fallback = new Intl.DateTimeFormat(undefined, {
			hour: 'numeric',
			minute: '2-digit',
			hour12: true,
		}).format(date)
		return { time: fallback, zone: timeZone ?? 'Local' }
	}
}
