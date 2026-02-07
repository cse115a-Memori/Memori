import {
	checkPermissions,
	getCurrentPosition,
	requestPermissions,
} from '@tauri-apps/plugin-geolocation'
import { errAsync, okAsync, ResultAsync } from 'neverthrow'
import { browser } from '$app/environment'

export type LocationError = {
	type: 'not-available' | 'permission-denied' | 'unknown'
	message: string
}

type LocationPosition = Awaited<ReturnType<typeof getCurrentPosition>>

const normalizeError = (error: unknown): LocationError => {
	if (error instanceof Error) {
		return { type: 'unknown', message: error.message }
	}

	return { type: 'unknown', message: 'Unknown geolocation error.' }
}

const permissionDenied = (): ResultAsync<void, LocationError> =>
	errAsync({
		type: 'permission-denied',
		message: 'Location permission denied.',
	})

const ensurePermission = (): ResultAsync<void, LocationError> => {
	return ResultAsync.fromPromise(checkPermissions(), normalizeError).andThen(
		permissions => {
			if (
				permissions.location === 'prompt' ||
				permissions.location === 'prompt-with-rationale'
			) {
				return ResultAsync.fromPromise(
					requestPermissions(['location']),
					normalizeError
				).andThen(updated =>
					updated.location === 'granted' ? okAsync(undefined) : permissionDenied()
				)
			}

			return permissions.location === 'granted'
				? okAsync(undefined)
				: permissionDenied()
		}
	)
}

export const getCurrentLocation = (): ResultAsync<LocationPosition, LocationError> => {
	if (!browser) {
		return errAsync({
			type: 'not-available',
			message: 'Geolocation is only available in the app.',
		})
	}

	return ensurePermission().andThen(() =>
		ResultAsync.fromPromise(
			getCurrentPosition({
				enableHighAccuracy: true,
				timeout: 10000,
				maximumAge: 0,
			}),
			normalizeError
		)
	)
}
