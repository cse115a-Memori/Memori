import { RuneStore } from '@tauri-store/svelte'

import type { UserInfo } from '@/tauri'

export type AuthProvider = 'google' | 'github' | 'twitch'
type ProviderUsers = Partial<Record<AuthProvider, UserInfo>>
export type AuthState = { usersByProvider: ProviderUsers }

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

export const authState = authStore.state

let startPromise: Promise<void> | null = null

export function startAuthStore(): Promise<void> {
	startPromise ??= authStore.start().catch(error => {
		startPromise = null
		throw error
	})

	return startPromise
}
