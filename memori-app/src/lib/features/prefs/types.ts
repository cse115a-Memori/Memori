import type { PermissionStatus, Position } from '@tauri-apps/plugin-geolocation'
import type { SystemOptions } from '@/features/prefs/system-options'

export type LocationStatus = PermissionStatus['location'] | 'not-available'

export type PrefsState = {
	locationStatus: LocationStatus
	lastKnownLocation: Position | null
	onboarded: boolean
	lastKnownDeviceId: string | null
	systemOptions: SystemOptions
	name: string
}
