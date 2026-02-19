import { RuneStore } from '@tauri-store/svelte'

import type { UserInfo } from '@/tauri'

export type AuthProvider = 'google' | 'github' | 'twitch'
type ProviderUsers = Partial<Record<AuthProvider, UserInfo>>
type AuthState = { usersByProvider: ProviderUsers }

const initialAuthState: AuthState = {
	usersByProvider: {},
}

const authStore = new RuneStore<AuthState>('auth', initialAuthState, {
	autoStart: false,
	saveOnChange: true,
	hooks: {
		error: error => {
			console.error('Auth store error:', error)
		},
	},
})

let startPromise: Promise<void> | null = null

async function ensureAuthStoreStarted(): Promise<void> {
	startPromise ??= authStore.start()

	try {
		await startPromise
	} catch (error) {
		startPromise = null
		return Promise.reject(error)
	}
}

export async function getUser(provider: AuthProvider): Promise<UserInfo | null> {
	await ensureAuthStoreStarted()
	return authStore.state.usersByProvider[provider] ?? null
}

export async function setUser(provider: AuthProvider, user: UserInfo): Promise<void> {
	await ensureAuthStoreStarted()
	authStore.state.usersByProvider = {
		...authStore.state.usersByProvider,
		[provider]: user,
	}
}

export async function removeUser(provider: AuthProvider): Promise<void> {
	await ensureAuthStoreStarted()
	const nextUsers = { ...authStore.state.usersByProvider }
	delete nextUsers[provider]
	authStore.state.usersByProvider = nextUsers
}
