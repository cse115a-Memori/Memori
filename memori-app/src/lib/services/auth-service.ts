import { ResultAsync } from 'neverthrow'
import { type AuthProvider, authState, startAuthStore } from '@/stores/auth-store'
import { type AppError, commands, toCmdError, tryCmd, type UserInfo } from '@/tauri'

export type { AuthProvider }

export function login(provider: AuthProvider): ResultAsync<UserInfo, AppError> {
	return tryCmd(commands.loginWithProvider(provider)).andThen(user =>
		ResultAsync.fromPromise(startAuthStore(), toCmdError).map(() => {
			authState.usersByProvider = {
				...authState.usersByProvider,
				[provider]: user,
			}
			return user
		})
	)
}

export function getUser(
	provider: AuthProvider
): ResultAsync<UserInfo | null, AppError> {
	return ResultAsync.fromPromise(startAuthStore(), toCmdError).map(
		() => authState.usersByProvider[provider] ?? null
	)
}

export function logout(provider: AuthProvider): ResultAsync<void, AppError> {
	return ResultAsync.fromPromise(startAuthStore(), toCmdError).map(() => {
		const nextUsers = { ...authState.usersByProvider }
		delete nextUsers[provider]
		authState.usersByProvider = nextUsers
		return undefined
	})
}
