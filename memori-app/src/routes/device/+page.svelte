<script lang="ts">
	import { onMount } from 'svelte'

	import WidgetEditor from '@/components/layout/WidgetEditor.svelte'
	import { authState } from '@/features/auth/store'
	// import { connState } from '@/features/connection'
	import { refreshLocationState, requestLocationState } from '@/features/prefs/service'
	import { prefsState } from '@/features/prefs/store'
	import { playFailedSound, playSuccessSound } from '@/features/sound'
	import { commands, type DeviceMode, tryCmd } from '@/tauri'
	import { Button } from '$lib/components/ui/button/index.js'
	import * as Field from '$lib/components/ui/field/index.js'
	import { Input } from '$lib/components/ui/input/index.js'

	const connState = $state({
		isConnected: false,
	})

	type PendingAction =
		| 'connect'
		| 'disconnect'
		| 'battery'
		| 'name'
		| 'temp'
		| 'location'
		| 'bustime'

	type DeviceResult = number | string | null

	let pendingOp = $state<PendingAction | null>(null)
	let actionResult = $state<DeviceResult>(null)

	$inspect(connState?.isConnected)

	let name = $state('')
	let city = $state('')
	let deviceMode: DeviceMode = $state('RealDevice')
	const isBusy = $derived(pendingOp !== null)
	const locationStatus = $derived(prefsState.locationStatus)

	function toErrMessage(err: unknown): string {
		if (typeof err === 'string') return err
		if (err instanceof Error) return err.message
		return String(err)
	}

	async function runPendingAction(
		action: PendingAction,
		task: () => Promise<void>
	): Promise<void> {
		pendingOp = action
		try {
			await task()
		} finally {
			pendingOp = null
		}
	}

	async function withCurrentLocation(
		onSuccess: (lat: number, lon: number) => Promise<void>
	): Promise<void> {
		const pos = await requestLocationState()
		if (!pos) {
			actionResult = 'Location permission is required'
			return
		}

		await onSuccess(pos.coords.latitude, pos.coords.longitude)
	}

	async function syncConnState() {
		await tryCmd(commands.isConnected()).match(
			nextIsConnected => {
				connState.isConnected = nextIsConnected
			},
			error => {
				actionResult = `Failed to read connection state: ${error}`
			}
		)
	}

	onMount(() => {
		void initPage()
	})

	async function initPage() {
		try {
			await Promise.all([syncConnState(), refreshLocationState()])
		} catch (err) {
			actionResult = `Initialization failed: ${toErrMessage(err)}`
		}
	}

	$inspect(authState)

	async function requestLocationAccess() {
		await runPendingAction('location', async () => {
			try {
				const pos = await requestLocationState()
				if (!pos) {
					actionResult = 'Location access was not granted'
					return
				}
				actionResult = `Location updated: ${pos.coords.latitude.toFixed(6)}, ${pos.coords.longitude.toFixed(6)}`
			} catch (err) {
				actionResult = `Location update failed: ${toErrMessage(err)}`
			}
		})
	}

	async function connect() {
		await runPendingAction('connect', async () => {
			await tryCmd(commands.connectDevice(deviceMode)).match(
				() => {
					connState.isConnected = true
					playSuccessSound()
					actionResult = `Connected to ${deviceMode}`
				},
				error => {
					playFailedSound()
					actionResult = `Connection failed: ${error}`
				}
			)
		})
	}

	async function disconnect() {
		await runPendingAction('disconnect', async () => {
			await tryCmd(commands.disconnectDevice()).match(
				() => {
					connState.isConnected = false
					actionResult = 'Disconnected'
				},
				error => {
					actionResult = `Disconnect failed: ${error}`
				}
			)
		})
	}

	async function getBattery() {
		await runPendingAction('battery', async () => {
			await tryCmd(commands.getBattery()).match(
				level => {
					actionResult = level
				},
				error => {
					actionResult = `Battery request failed: ${error}`
				}
			)
		})
	}

	async function sendName() {
		await runPendingAction('name', async () => {
			await tryCmd(commands.sendName(name)).match(
				() => {
					actionResult = 'Name sent'
				},
				error => {
					actionResult = `Send name failed: ${error}`
				}
			)
		})
	}

	async function sendTemp() {
		await runPendingAction('temp', async () => {
			await withCurrentLocation(async (lat, lon) => {
				await tryCmd(commands.sendTemp(lat, lon)).match(
					() => {
						actionResult = 'Weather sent'
					},
					error => {
						actionResult = `Send weather failed: ${error}`
					}
				)
			})
		})
	}

	async function sendBustime() {
		await runPendingAction('bustime', async () => {
			try {
				await withCurrentLocation(async (lat, lon) => {
					await tryCmd(commands.sendBustime(lat, lon)).match(
						data => {
							actionResult = data
						},
						error => {
							actionResult = `Bustime request failed: ${error}`
						}
					)
				})
			} catch (err) {
				actionResult = `Bustime request failed: ${toErrMessage(err)}`
			}
		})
	}
</script>

<Button onclick={()=> connState.isConnected = !connState.isConnected}
	>Dev Toggle</Button
>

{#if !connState.isConnected}
	<section class="space-y-6">
		<h1 class="text-2xl font-semibold">Device Controls</h1>

		<Field.Field orientation="horizontal" class="justify-center mx-auto max-w-xs">
			<Field.Label for="device-mode" class="sr-only">Device Mode</Field.Label>
			<select
				id="device-mode"
				bind:value={deviceMode}
				disabled={isBusy || connState.isConnected}
				class="border rounded px-3 py-2"
			>
				<option value="RealDevice">Real Device (Bluetooth)</option>
				<option value="Simulator">Simulator (TCP)</option>
			</select>

			<Button
				variant="outline"
				onclick={connState.isConnected ? disconnect : connect}
				disabled={isBusy}
			>
				{#if pendingOp === 'connect'}
					Connecting...
				{:else if pendingOp === 'disconnect'}
					Disconnecting...
				{:else}
					{connState.isConnected ? 'Disconnect' : 'Connect'}
				{/if}
			</Button>
			<Button variant="outline" onclick={disconnect} disabled={isBusy}>
				Disconnect
			</Button>
		</Field.Field>

		<Field.Field orientation="horizontal" class="justify-center mx-auto max-w-xs">
			<Field.Label for="name-input" class="sr-only">Name</Field.Label>
			<Input id="name-input" placeholder="Enter a name..." bind:value={name} />
			<Button variant="outline" onclick={sendName} disabled={isBusy || !name.trim()}>
				{pendingOp === 'name' ? 'Sending...' : 'Send Name'}
			</Button>
		</Field.Field>

		<Field.Field orientation="horizontal" class="justify-center mx-auto max-w-xs">
			<Field.Label for="city-input" class="sr-only">City</Field.Label>
			<Input id="city-input" placeholder="Enter a city..." bind:value={city} />
			<Button variant="outline" onclick={sendTemp} disabled={isBusy || !city.trim()}>
				{pendingOp === 'temp' ? 'Sending...' : 'Send Weather'}
			</Button>
		</Field.Field>

		<div class="flex justify-center gap-3">
			<Button variant="outline" onclick={getBattery} disabled={isBusy}>
				{pendingOp === 'battery' ? 'Checking...' : 'Device Battery'}
			</Button>

			<Button variant="outline" onclick={sendBustime} disabled={isBusy}>
				{pendingOp === 'bustime' ? 'Sending...' : 'Send Bustime'}
			</Button>
		</div>

		<section class="space-y-1 text-center text-sm">
			<p>Location Status: {locationStatus}</p>
			<p>
				Last Known Location:
				{prefsState.lastKnownLocation
				? `${prefsState.lastKnownLocation.coords.latitude.toFixed(6)}, ${prefsState.lastKnownLocation.coords.longitude.toFixed(6)}`
				: 'None'}
			</p>
		</section>

		{#if locationStatus !== 'granted' && locationStatus !== 'not-available'}
			<div class="flex justify-center">
				<Button variant="outline" onclick={requestLocationAccess} disabled={isBusy}>
					{pendingOp === 'location' ? 'Requesting...' : 'Enable Location Access'}
				</Button>
			</div>
		{/if}

		{#if actionResult !== null}
			<p class="text-center text-sm">{actionResult}</p>
		{/if}
	</section>
{:else}
	<WidgetEditor />
{/if}
