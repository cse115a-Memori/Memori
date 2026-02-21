import {
	checkPermissions,
	getCurrentPosition,
	type Position,
	requestPermissions,
} from '@tauri-apps/plugin-geolocation'
import { appState, startAppStore } from '@/stores/app-store'
import { isMobilePlatform } from '@/utils'

async function readLocation(requestAccess: boolean): Promise<Position | null> {
	if (!isMobilePlatform()) {
		appState.locationStatus = 'not-available'
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

	appState.locationStatus = permissions.location
	if (permissions.location !== 'granted') {
		return null
	}

	try {
		const position = await getCurrentPosition()
		appState.lastKnownLocation = position
		return position
	} catch {
		return null
	}
}

export async function refreshLocationState(): Promise<Position | null> {
	await startAppStore()
	const fallbackPosition = appState.lastKnownLocation

	try {
		const latestPosition = await readLocation(false)
		return latestPosition ?? fallbackPosition
	} catch {
		return fallbackPosition
	}
}

export async function requestLocationState(): Promise<Position | null> {
	await startAppStore()
	return readLocation(true)
}
