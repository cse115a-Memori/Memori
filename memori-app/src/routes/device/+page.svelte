<script lang="ts">
	import { onMount } from 'svelte'

	import WidgetEditor from '@/components/layout/WidgetEditor.svelte'
	import { connState, connectDevice, disconnectDevice, syncConnectionState } from '@/features/connection'
	import { refreshLocationState, requestLocationState } from '@/features/prefs/service'
	import { prefsState } from '@/features/prefs/store'
	import { playFailedSound, playSuccessSound } from '@/features/sound'
	import { commands, toCmdError, type DeviceMode, tryCmd } from '@/tauri'
	import { Button } from '$lib/components/ui/button/index.js'
	import * as Field from '$lib/components/ui/field/index.js'
	import { Input } from '$lib/components/ui/input/index.js'
	import * as NativeSelect from '$lib/components/ui/native-select/index.js'

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

	let name = $state('')
	let deviceMode: DeviceMode = $state('RealDevice')
	const isBusy = $derived(pendingOp !== null)
	const locationStatus = $derived(prefsState.locationStatus)

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
		await syncConnectionState().match(
			() => undefined,
			error => {
				actionResult = `Failed to read connection state: ${error}`
			}
		)
	}

	onMount(() => {
		void initPage()
	})

	async function initPage() {
		await Promise.allSettled([
			syncConnState(),
			refreshLocationState().catch(error => {
				actionResult = `Location refresh failed: ${toCmdError(error)}`
			}),
		])
	}

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
				actionResult = `Location update failed: ${toCmdError(err)}`
			}
		})
	}

	async function connect() {
		await runPendingAction('connect', async () => {
			await connectDevice(deviceMode).match(
				() => {
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
			await disconnectDevice().match(
				() => {
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
				actionResult = `Bustime request failed: ${toCmdError(err)}`
			}
		})
	}
</script>

{#if !connState.isConnected}
	<section class="space-y-6">
		<header class="space-y-2">
			<h1 class="text-2xl font-semibold tracking-tight">Device Controls</h1>
			<p class="text-sm text-muted-foreground">
				Use this screen as the operational fallback after onboarding or whenever you need to
				reconnect and resend payloads.
			</p>
		</header>

		<section class="rounded-2xl border bg-card p-4 shadow-sm">
			<div class="flex items-center justify-between gap-3">
				<div>
					<p class="text-sm font-medium text-foreground">Connection</p>
					<p class="text-sm text-muted-foreground">
						{connState.isConnected ? 'Connected and ready.' : 'Waiting for device.'}
					</p>
				</div>
				<div class="text-sm text-muted-foreground">Mode</div>
			</div>

			<div class="mt-4 flex flex-col gap-3 sm:flex-row">
				<NativeSelect.Root
					id="device-mode"
					bind:value={deviceMode}
					disabled={isBusy || connState.isConnected}
					class="h-11 flex-1"
				>
					<NativeSelect.Option value="RealDevice">Real Device (Bluetooth)</NativeSelect.Option>
					<NativeSelect.Option value="Simulator">Simulator (TCP)</NativeSelect.Option>
				</NativeSelect.Root>

				<Button
					variant="outline"
					class="h-11"
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
			</div>
		</section>

		<Field.Field orientation="horizontal" class="mx-auto max-w-xl justify-center">
			<Field.Label for="name-input" class="sr-only">Name</Field.Label>
			<Input id="name-input" placeholder="Enter a name..." bind:value={name} />
			<Button variant="outline" onclick={sendName} disabled={isBusy || !name.trim()}>
				{pendingOp === 'name' ? 'Sending...' : 'Send Name'}
			</Button>
		</Field.Field>

		<div class="grid gap-3 sm:grid-cols-3">
			<Button variant="outline" onclick={getBattery} disabled={isBusy}>
				{pendingOp === 'battery' ? 'Checking...' : 'Device Battery'}
			</Button>

			<Button variant="outline" onclick={sendTemp} disabled={isBusy}>
				{pendingOp === 'temp' ? 'Sending...' : 'Send Weather'}
			</Button>

			<Button variant="outline" onclick={sendBustime} disabled={isBusy}>
				{pendingOp === 'bustime' ? 'Sending...' : 'Send Bustime'}
			</Button>
		</div>

		<section class="space-y-2 rounded-2xl border bg-card p-4 text-sm shadow-sm">
			<p>Location Status: {locationStatus}</p>
			<p>
				Last Known Location:
				{prefsState.lastKnownLocation
					? `${prefsState.lastKnownLocation.coords.latitude.toFixed(6)}, ${prefsState.lastKnownLocation.coords.longitude.toFixed(6)}`
					: 'None'}
			</p>

			{#if locationStatus !== 'granted' && locationStatus !== 'not-available'}
				<Button variant="outline" onclick={requestLocationAccess} disabled={isBusy}>
					{pendingOp === 'location' ? 'Requesting...' : 'Enable Location Access'}
				</Button>
			{/if}
		</section>

		{#if actionResult !== null}
			<p class="text-sm text-muted-foreground">
				{typeof actionResult === 'number' ? `Battery: ${actionResult}%` : actionResult}
			</p>
		{/if}
	</section>
{:else}
	<WidgetEditor />
{/if}
