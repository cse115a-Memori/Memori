import { commands, type DeviceMode, tryCmd } from '@/tauri'

import { connState } from './store.svelte'
import { prefsState } from '@/features/prefs/store'


export function syncConnectionState() {
	return tryCmd(commands.isConnected()).map(isConnected => {
		connState.isConnected = isConnected
		return isConnected
	})
}

export function retryConnection() {
  if (prefsState.lastKnownDeviceId === '') {
    console.log("Could not find device code for retryConnection")
    return
  }
  
  return connectDevice('RealDevice', connState.deviceCode)
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
