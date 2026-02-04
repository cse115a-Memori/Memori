import {
	type AdapterState,
	type BleDevice,
	checkPermissions,
	connect,
	disconnect,
	getAdapterState,
	getConnectionUpdates,
	getScanningUpdates,
	readString,
	sendString,
	startScan,
	stopScan,
	subscribeString,
	unsubscribe,
} from '@mnlphlp/plugin-blec'
import { invoke } from '@tauri-apps/api/core'
import { saveAll } from '@tauri-store/svelte'
import { onMount } from 'svelte'

import { store as bleStore } from '$lib/stores/ble.svelte'

import {
	CHARACTERISTIC_UUID,
	DEVICE_NAME_UUID,
	GAP_SERVICE_UUID,
	SERVICE_UUID,
	SERVICE2_UUID,
} from './ble-uuids'

export function createBlePageModel() {
	const state = $state({
		devices: [] as BleDevice[],
		connected: false,
		scanning: false,
		sendData: '',
		recvData: '',
		rustTest: false,
		sendData2: '',
		recvData2: '',
		notifyData: '',
		notifyData2: '',
		showServices: false,
		adapterState: 'Unknown' as AdapterState,
		permissionsGranted: false,
		resolvingNames: false,
	})

	let skipInitialSave = true

	onMount(async () => {
		await getConnectionUpdates(value => {
			state.connected = value
		})
		await getScanningUpdates(value => {
			console.log('Scanning:', value)
			state.scanning = value
			if (!value) {
				void resolveMissingNames()
			}
		})
	})

	$effect(() => {
		void bleStore.state.showAllDevices
		if (skipInitialSave) {
			skipInitialSave = false
			return
		}
		void saveAll()
	})

	async function resolveMissingNames() {
		if (state.resolvingNames || state.connected || state.scanning) return
		state.resolvingNames = true

		let didUpdate = false

		for (const device of state.devices) {
			if (device.name || bleStore.state.resolvedNames[device.address]) continue

			try {
				await connect(device.address, () => {})
				const name = await readString(DEVICE_NAME_UUID, GAP_SERVICE_UUID)
				if (name) {
					bleStore.state.resolvedNames[device.address] = name
					didUpdate = true
				}
			} catch (error) {
				console.warn('Failed to resolve device name', device.address, error)
			} finally {
				try {
					await disconnect()
				} catch (error) {
					console.warn('Failed to disconnect after name lookup', error)
				}
			}
		}

		if (didUpdate) {
			await saveAll()
		}
		state.resolvingNames = false
	}

	async function toggleNotifyPrimary() {
		if (state.notifyData) {
			unsubscribe(CHARACTERISTIC_UUID)
			state.notifyData = ''
		} else {
			subscribeString(CHARACTERISTIC_UUID, (data: string) => {
				state.notifyData = data
			})
		}
	}

	async function toggleNotifySecondary() {
		if (state.notifyData2) {
			unsubscribe(CHARACTERISTIC_UUID)
			state.notifyData2 = ''
		} else {
			subscribeString(CHARACTERISTIC_UUID, (data: string) => {
				state.notifyData2 = data
			})
		}
	}

	async function testRust() {
		try {
			const resp = await invoke<boolean>('test')
			state.rustTest = resp
		} catch (error) {
			console.error(error)
		}
	}

	async function checkState() {
		state.adapterState = await getAdapterState()
	}

	async function checkPermission(askIfDenied = true) {
		state.permissionsGranted = await checkPermissions(askIfDenied)
	}

	async function startScanDevices(withIBeacons = false) {
		startScan(
			(dev: BleDevice[]) => {
				state.devices = dev
			},
			10000,
			withIBeacons
		)
	}

	async function connectDevice(device: BleDevice) {
		await connect(device.address, () => console.log('disconnected'))
		console.log('connect command returned')
	}

	function stopScanDevices() {
		stopScan()
	}

	function disconnectDevice() {
		void disconnect()
	}

	function sendPrimary() {
		void sendString(CHARACTERISTIC_UUID, state.sendData, 'withResponse', SERVICE_UUID)
	}

	async function readPrimary() {
		state.recvData = await readString(CHARACTERISTIC_UUID, SERVICE_UUID)
	}

	function sendSecondary() {
		void sendString(CHARACTERISTIC_UUID, state.sendData2, 'withResponse', SERVICE2_UUID)
	}

	async function readSecondary() {
		state.recvData2 = await readString(CHARACTERISTIC_UUID, SERVICE2_UUID)
	}

	return {
		state,
		actions: {
			checkPermission,
			checkState,
			connectDevice,
			disconnectDevice,
			readPrimary,
			readSecondary,
			sendPrimary,
			sendSecondary,
			startScanDevices,
			stopScanDevices,
			testRust,
			toggleNotifyPrimary,
			toggleNotifySecondary,
		},
	}
}
