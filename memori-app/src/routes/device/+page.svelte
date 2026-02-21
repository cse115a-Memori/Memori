<script lang="ts">
  import { onMount } from 'svelte'
  import {
    refreshLocationState,
    requestLocationState,
  } from '@/services/location-service'
  import { appState } from '@/stores/app-store'
  import { commands, type DeviceMode, tryCmd } from '@/tauri'
  import { Button } from '$lib/components/ui/button/index.js'
  import * as Field from '$lib/components/ui/field/index.js'
  import { Input } from '$lib/components/ui/input/index.js'

  type PendingAction =
    | 'connect'
    | 'disconnect'
    | 'battery'
    | 'name'
    | 'temp'
    | 'location'
    | 'bustime'

  type DeviceResult = number | string | null

  let connected = $state(false)
  let pending = $state<PendingAction | null>(null)
  let result = $state<DeviceResult>(null)

  let name = $state('')
  let city = $state('')
  let selectedMode: DeviceMode = $state('RealDevice')
  const isBusy = $derived(pending !== null)
  const locationStatus = $derived(appState.locationStatus)

  async function syncConnectionState() {
    await tryCmd(commands.isConnected()).match(
      (isConnected) => {
        connected = isConnected
      },
      (error) => {
        result = `Failed to read connection state: ${error}`
      }
    )
  }

  onMount(() => {
    void initializePage()
  })

  async function initializePage() {
    try {
      await Promise.all([syncConnectionState(), refreshLocationState()])
    } catch (error) {
      result = `Initialization failed: ${
        typeof error === 'string'
          ? error
          : error instanceof Error
            ? error.message
            : String(error)
      }`
    }
  }

  async function enableLocationAccess() {
    pending = 'location'
    try {
      const position = await requestLocationState()
      if (!position) {
        result = 'Location access was not granted'
        return
      }
      result = `Location updated: ${position.coords.latitude.toFixed(6)}, ${position.coords.longitude.toFixed(6)}`
    } catch (error) {
      result = `Location update failed: ${
        typeof error === 'string'
          ? error
          : error instanceof Error
            ? error.message
            : String(error)
      }`
    } finally {
      pending = null
    }
  }

  async function connect() {
    pending = 'connect'
    try {
      await tryCmd(commands.connectDevice(selectedMode)).match(
        () => {
          connected = true
          result = `Connected to ${selectedMode}`
        },
        (error) => {
          result = `Connection failed: ${error}`
        }
      )
    } finally {
      pending = null
    }
  }

  async function disconnect() {
    pending = 'disconnect'
    try {
      await tryCmd(commands.disconnectDevice()).match(
        () => {
          connected = false
          result = 'Disconnected'
        },
        (error) => {
          result = `Disconnect failed: ${error}`
        }
      )
    } finally {
      pending = null
    }
  }

  async function getBattery() {
    pending = 'battery'
    try {
      await tryCmd(commands.getBattery()).match(
        (level) => {
          result = level
        },
        (error) => {
          result = `Battery request failed: ${error}`
        }
      )
    } finally {
      pending = null
    }
  }

  async function sendName() {
    pending = 'name'
    try {
      await tryCmd(commands.sendName(name)).match(
        () => {
          result = 'Name sent'
        },
        (error) => {
          result = `Send name failed: ${error}`
        }
      )
    } finally {
      pending = null
    }
  }

  async function sendTemp() {
    pending = 'temp'
    try {
      await tryCmd(commands.sendTemp(city)).match(
        () => {
          result = 'Weather sent'
        },
        (error) => {
          result = `Send weather failed: ${error}`
        }
      )
    } finally {
      pending = null
    }
  }

  async function sendBustime() {
    pending = 'bustime'

    try {
      const position = await requestLocationState()
      if (!position) {
        result = 'Location permission is required'
        return
      }

      await tryCmd(
        commands.sendBustime(
          position.coords.latitude,
          position.coords.longitude
        )
      ).match(
        (data) => {
          result = data
        },
        (error) => {
          result = `Bustime request failed: ${error}`
        }
      )
    } catch (error) {
      result = `Bustime request failed: ${
        typeof error === 'string'
          ? error
          : error instanceof Error
            ? error.message
            : String(error)
      }`
    } finally {
      pending = null
    }
  }
</script>

<main class="space-y-6">
  <h1 class="text-2xl font-semibold">Device Controls</h1>

  <Field.Field orientation="horizontal" class="justify-center mx-auto max-w-xs">
    <Field.Label for="device-mode" class="sr-only">Device Mode</Field.Label>
    <select
      id="device-mode"
      bind:value={selectedMode}
      disabled={isBusy || connected}
      class="border rounded px-3 py-2"
    >
      <option value="RealDevice">Real Device (Bluetooth)</option>
      <option value="Simulator">Simulator (TCP)</option>
    </select>

    <Button
      variant="outline"
      onclick={connected ? disconnect : connect}
      disabled={isBusy}
    >
      {#if pending === 'connect'}
        Connecting...
      {:else if pending === 'disconnect'}
        Disconnecting...
      {:else}
        {connected ? 'Disconnect' : 'Connect'}
      {/if}
    </Button>
  </Field.Field>

  <Field.Field orientation="horizontal" class="justify-center mx-auto max-w-xs">
    <Field.Label for="name-input" class="sr-only">Name</Field.Label>
    <Input id="name-input" placeholder="Enter a name..." bind:value={name} />
    <Button
      variant="outline"
      onclick={sendName}
      disabled={isBusy || !name.trim()}
    >
      {pending === 'name' ? 'Sending...' : 'Send Name'}
    </Button>
  </Field.Field>

  <Field.Field orientation="horizontal" class="justify-center mx-auto max-w-xs">
    <Field.Label for="city-input" class="sr-only">City</Field.Label>
    <Input id="city-input" placeholder="Enter a city..." bind:value={city} />
    <Button
      variant="outline"
      onclick={sendTemp}
      disabled={isBusy || !city.trim()}
    >
      {pending === 'temp' ? 'Sending...' : 'Send Weather'}
    </Button>
  </Field.Field>

  <div class="flex justify-center gap-3">
    <Button variant="outline" onclick={getBattery} disabled={isBusy}>
      {pending === 'battery' ? 'Checking...' : 'Device Battery'}
    </Button>

    <Button variant="outline" onclick={sendBustime} disabled={isBusy}>
      {pending === 'bustime' ? 'Sending...' : 'Send Bustime'}
    </Button>
  </div>

  <section class="space-y-1 text-center text-sm">
    <p>Location Status: {locationStatus}</p>
    <p>
      Last Known Location:
      {appState.lastKnownLocation
        ? `${appState.lastKnownLocation.coords.latitude.toFixed(6)}, ${appState.lastKnownLocation.coords.longitude.toFixed(6)}`
        : 'None'}
    </p>
  </section>

  {#if locationStatus !== 'granted' && locationStatus !== 'not-available'}
    <div class="flex justify-center">
      <Button
        variant="outline"
        onclick={enableLocationAccess}
        disabled={isBusy}
      >
        {pending === 'location' ? 'Requesting...' : 'Enable Location Access'}
      </Button>
    </div>
  {/if}

  {#if result !== null}
    <p class="text-center text-sm">{result}</p>
  {/if}
</main>
