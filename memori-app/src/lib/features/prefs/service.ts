import {
	checkPermissions,
	getCurrentPosition,
	type Position,
	requestPermissions,
} from '@tauri-apps/plugin-geolocation'
import { prefsState, startPrefsStore } from '@/features/prefs/store.ts'
import { isMobilePlatform } from '@/utils.ts'

async function readLocation(requestAccess: boolean): Promise<Position | null> {
	if (!isMobilePlatform()) {
		prefsState.locationStatus = 'not-available'
		return null
	}

	let permissions = await checkPermissions()
	if (
		requestAccess &&
		(permissions.location === 'prompt' ||
			permissions.location === 'prompt-with-rationale')
	) {
		permissions = await requestPermissions(['location'])
	}

	prefsState.locationStatus = permissions.location
	if (permissions.location !== 'granted') {
		return null
	}

	try {
		const position = await getCurrentPosition()
		prefsState.lastKnownLocation = position
		return position
	} catch {
		return null
	}
}

export async function refreshLocationState(): Promise<Position | null> {
	await startPrefsStore()
	const fallbackPosition = prefsState.lastKnownLocation

	try {
		const latestPosition = await readLocation(false)
		return latestPosition ?? fallbackPosition
	} catch {
		return fallbackPosition
	}
}

export async function requestLocationState(): Promise<Position | null> {
	await startPrefsStore()
	return readLocation(true)
}
