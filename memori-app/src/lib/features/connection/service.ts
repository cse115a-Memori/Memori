import { commands, tryCmd, type DeviceMode } from '@/tauri'

import { connState } from './store.svelte'

export function syncConnectionState() {
	return tryCmd(commands.isConnected()).map((isConnected) => {
		connState.isConnected = isConnected
		return isConnected
	})
}

export function connectDevice(mode: DeviceMode) {
	return tryCmd(commands.connectDevice(mode)).map(() => {
		connState.isConnected = true
	})
}

export function disconnectDevice() {
	return tryCmd(commands.disconnectDevice()).map(() => {
		connState.isConnected = false
	})
}
