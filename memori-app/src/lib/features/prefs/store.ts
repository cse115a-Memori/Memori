import { RuneStore } from '@tauri-store/svelte'
import { getSystemOptions } from '@/features/prefs/system-options'
import type { PrefsState } from '@/features/prefs/types'

export type { LocationStatus, PrefsState } from '@/features/prefs/types'

const initialPrefsState: PrefsState = {
	locationStatus: 'prompt',
	lastKnownLocation: null,
	onboarded: false,
	lastKnownDeviceId: null,
	lastKnownBleAddress: null,
	systemOptions: getSystemOptions(),
	name: '',
}

const prefsStore = new RuneStore<PrefsState>('prefs', initialPrefsState, {
	autoStart: false,
	saveOnChange: true,
	hooks: {
		error: error => {
			console.error('Prefs store error:', error)
		},
	},
})

export const prefsState = prefsStore.state

let startPromise: Promise<void> | null = null

export function startPrefsStore(): Promise<void> {
	startPromise ??= prefsStore.start().catch(error => {
		startPromise = null
		throw error
	})

	return startPromise
}
