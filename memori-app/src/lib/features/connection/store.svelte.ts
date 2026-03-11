import type { ConnectionState } from '@/features/connection/types'

const initialConnectionState: ConnectionState = {
	isConnected: false,
	deviceCode: '',
}

export const connState = $state<ConnectionState>(initialConnectionState)

// export const connectionState = connectionStore.state

// let startPromise: Promise<void> | null = null

// export function startConnectionStore(): Promise<void> {
// 	startPromise ??= connectionStore.start().catch(error => {
// 		startPromise = null
// 		throw error
// 	})

// 	return startPromise
// }
