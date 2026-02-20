import { appState, type LocationStatus, startAppStore } from '$lib/stores/app-store'

export async function setLocationShared(status: LocationStatus): Promise<void> {
	await startAppStore()
	appState.locationStatus = status
}

export async function setOnboarded(onboarded: boolean): Promise<void> {
	await startAppStore()
	appState.onboarded = onboarded
}

export async function setLastKnownDeviceId(
	lastKnownDeviceId: string | null
): Promise<void> {
	await startAppStore()
	appState.lastKnownDeviceId = lastKnownDeviceId
}

export async function forgetDevice(): Promise<void> {
	await startAppStore()
	appState.lastKnownDeviceId = null
}
