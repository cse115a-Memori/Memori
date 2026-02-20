import type { PermissionStatus, Position } from '@tauri-apps/plugin-geolocation'
import { RuneStore } from '@tauri-store/svelte'

export type LocationStatus = PermissionStatus['location'] | 'not-available'

export type AppState = {
	locationStatus: LocationStatus
	lastKnownLocation: Position | null
	onboarded: boolean
	lastKnownDeviceId: string | null
}

const initialAppState: AppState = {
	locationStatus: 'prompt',
	lastKnownLocation: null,
	onboarded: false,
	lastKnownDeviceId: null,
}

const appStore = new RuneStore<AppState>('app-store', initialAppState, {
	autoStart: false,
	saveOnChange: true,
	hooks: {
		error: error => {
			console.error('App store error:', error)
		},
	},
})

export const appState = appStore.state

let startPromise: Promise<void> | null = null

export function startAppStore(): Promise<void> {
	startPromise ??= appStore.start().catch(error => {
		startPromise = null
		throw error
	})

	return startPromise
}
