import { okAsync, ResultAsync } from 'neverthrow'

import { type AppError, commands, toCmdError, tryCmd, type UserInfo } from '@/tauri'
import {
	type AuthProvider,
	getUser as getStoreUser,
	removeUser as removeStoreUser,
	setUser as setStoreUser,
} from '$lib/stores/auth-store'

export type { AuthProvider }

function fromStore<T>(promise: Promise<T>): ResultAsync<T, AppError> {
	return ResultAsync.fromPromise(promise, toCmdError)
}

export function login(provider: AuthProvider): ResultAsync<UserInfo, AppError> {
	return tryCmd(commands.loginWithProvider(provider)).andThen(user =>
		fromStore(setStoreUser(provider, user)).andThen(() => okAsync(user))
	)
}

export function getUser(
	provider: AuthProvider
): ResultAsync<UserInfo | null, AppError> {
	return fromStore(getStoreUser(provider))
}

export function logout(provider: AuthProvider): ResultAsync<void, AppError> {
	return fromStore(removeStoreUser(provider))
}
