import type { PermissionStatus, Position } from '@tauri-apps/plugin-geolocation'
import type { WidgetFrameEntry, WidgetFrameSignatures } from '@/model/widget-frame.ts'
import type { SystemOptions } from '@/services/system-service'
import type { MemoriWidget } from '@/tauri'

export type LocationStatus = PermissionStatus['location'] | 'not-available'

export type AppState = {
	locationStatus: LocationStatus
	lastKnownLocation: Position | null
	onboarded: boolean
	lastKnownDeviceId: string | null
	systemOptions: SystemOptions
	widgetPool: MemoriWidget[]
	widgetFrames: WidgetFrameEntry[]
	lastFlashedSignaturesByFrame: WidgetFrameSignatures[]
}
