import { commands, type DeviceMode, tryCmd } from '@/tauri'

import { connState } from './store.svelte'

export function syncConnectionState() {
	return tryCmd(commands.isConnected()).map(isConnected => {
		connState.isConnected = isConnected
		return isConnected
	})
}

export function connectDevice(mode: DeviceMode, code: string) {
	return tryCmd(commands.connectDevice(mode, code)).map(() => {
		connState.isConnected = true
		return connState.isConnected
	})
}

export function disconnectDevice() {
	return tryCmd(commands.disconnectDevice()).map(() => {
		connState.isConnected = false
		return connState.isConnected
	})
}
