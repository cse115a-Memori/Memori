import { RuneStore } from '@tauri-store/svelte'
import { ResultAsync } from 'neverthrow'

export type AppState = {
	locationShared: boolean
	onboarded: boolean
	lastKnownDeviceId: string | null
}

export type AppStoreError = string

const initialAppState: AppState = {
	locationShared: false,
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

const toStoreError = (error: unknown): AppStoreError =>
	error instanceof Error ? error.message : String(error)

let startPromise: Promise<void> | null = null

function ensureStoreStarted(): ResultAsync<void, AppStoreError> {
	startPromise ??= appStore.start()

	return ResultAsync.fromPromise(startPromise, toStoreError).mapErr(error => {
		startPromise = null
		return error
	})
}

function withStore<T>(
	fn: (store: typeof appStore) => T | Promise<T>
): ResultAsync<T, AppStoreError> {
	return ensureStoreStarted().andThen(() =>
		ResultAsync.fromPromise(Promise.resolve(fn(appStore)), toStoreError)
	)
}

export function getAppState(): ResultAsync<AppState, AppStoreError> {
	return withStore(store => ({ ...store.state }))
}

export function patchAppState(
	patch: Partial<AppState>
): ResultAsync<void, AppStoreError> {
	return withStore(store => {
		Object.assign(store.state, patch)
	})
}

export function forgetDevice(): ResultAsync<void, AppStoreError> {
	return patchAppState({ lastKnownDeviceId: null })
}
