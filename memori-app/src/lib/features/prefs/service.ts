import {
	checkPermissions,
	getCurrentPosition,
	type Position,
	requestPermissions,
} from '@tauri-apps/plugin-geolocation'
import { prefsState, startPrefsStore } from '@/features/prefs/store'
import { isMobilePlatform } from '@/utils'

async function readLocation(requestAccess: boolean): Promise<Position | null> {
	if (!isMobilePlatform()) {
		prefsState.locationStatus = 'not-available'
		return null
	}

	let perms = await checkPermissions()
	if (
		requestAccess &&
		(perms.location === 'prompt' || perms.location === 'prompt-with-rationale')
	) {
		perms = await requestPermissions(['location'])
	}

	prefsState.locationStatus = perms.location
	if (perms.location !== 'granted') {
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
	const fallbackPos = prefsState.lastKnownLocation

	try {
		const latestPos = await readLocation(false)
		return latestPos ?? fallbackPos
	} catch {
		return fallbackPos
	}
}

export async function requestLocationState(): Promise<Position | null> {
	await startPrefsStore()
	return readLocation(true)
}
