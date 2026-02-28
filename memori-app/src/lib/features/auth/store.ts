import { RuneStore } from '@tauri-store/svelte'

import type { AuthState } from '@/features/auth/types.ts'

export type { AuthProvider, AuthState } from '@/features/auth/types.ts'

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
