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
  if (prefsState.lastKnownDeviceId === '' || prefsState.lastKnownDeviceId === null) {
    console.log("Could not find device code for retryConnection")
    return
  }
  
  return connectDevice('RealDevice', prefsState.lastKnownDeviceId)
}

export function connectDevice(mode: DeviceMode, code: string) {
	return tryCmd(commands.connectDevice(mode, code, prefsState.lastKnownBleAddress)).map((address) => {
    connState.isConnected = true
    prefsState.lastKnownBleAddress = address
		return connState.isConnected
	})
}

export function disconnectDevice() {
	return tryCmd(commands.disconnectDevice()).map(() => {
		connState.isConnected = false
		return connState.isConnected
	})
}
