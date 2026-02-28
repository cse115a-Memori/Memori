import type { UserInfo } from '@/tauri'

export type AuthProvider = 'google' | 'github' | 'twitch'
export type ProviderUsers = Partial<Record<AuthProvider, UserInfo>>
export type AuthState = { usersByProvider: ProviderUsers }
