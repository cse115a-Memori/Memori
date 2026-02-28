export type CompactClock = { time: string; zone: string }

export function formatCompactClock(date: Date, timeZone?: string): CompactClock {
	try {
		const parts = new Intl.DateTimeFormat(undefined, {
			hour: 'numeric',
			minute: '2-digit',
			hour12: true,
			timeZone,
			timeZoneName: 'short',
		}).formatToParts(date)
		const hour = parts.find(part => part.type === 'hour')?.value ?? ''
		const minute = parts.find(part => part.type === 'minute')?.value ?? '00'
		const dayPeriod =
			parts.find(part => part.type === 'dayPeriod')?.value?.toUpperCase() ?? ''
		const zone = parts.find(part => part.type === 'timeZoneName')?.value ?? 'Local'

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
