import type { Clock } from '@/tauri'

export type CompactClock = { time: string; zone: string }

export function getCurrentClock(date = new Date()): Clock {
	return {
		hours: date.getHours(),
		minutes: date.getMinutes(),
		seconds: date.getSeconds(),
	}
}

export function formatCompactClock(date: Date): CompactClock {
	const time = new Intl.DateTimeFormat(undefined, {
		hour: '2-digit',
		minute: '2-digit',
		hour12: false,
	}).format(date)

	const zone =
		new Intl.DateTimeFormat(undefined, { timeZoneName: 'short' })
			.formatToParts(date)
			.find(part => part.type === 'timeZoneName')?.value ?? 'Local'

	return { time, zone }
}
