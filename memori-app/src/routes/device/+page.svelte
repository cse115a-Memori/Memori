<script lang="ts">
  import {
    checkPermissions,
    getCurrentPosition,
    requestPermissions,
  } from '@tauri-apps/plugin-geolocation'
  import { onMount } from 'svelte'
  import {
    commands,
    type DeviceMode,
    type Result as TauriResult,
    tryCmd,
  } from '@/tauri'
  import { Button } from '$lib/components/ui/button/index.js'
  import * as Field from '$lib/components/ui/field/index.js'
  import { Input } from '$lib/components/ui/input/index.js'

  type PendingAction =
    | 'connect'
    | 'disconnect'
    | 'battery'
    | 'name'
    | 'temp'
    | 'bustime'

  type DeviceResult = number | string | null

  let connected = $state(false)
  let pending = $state<PendingAction | null>(null)
  let result = $state<DeviceResult>(null)

  let name = $state('')
  let city = $state('')
  let selectedMode: DeviceMode = $state('RealDevice')

  const busy = $derived(pending !== null)

  function setResult(next: DeviceResult) {
    result = next
  }

  function isPending(action: PendingAction) {
    return pending === action
  }

  async function runAction<T>(
    action: PendingAction,
    command: Promise<TauriResult<T, string>>,
    onOk: (data: T) => void,
    errorPrefix: string
  ) {
    pending = action
    try {
      await tryCmd(command).match(
        (data) => {
          onOk(data)
        },
        (error) => {
          setResult(`${errorPrefix}: ${error}`)
        }
      )
    } finally {
      pending = null
    }
  }

  async function syncConnectionState() {
    await tryCmd(commands.isConnected()).match(
      (isConnected) => {
        connected = isConnected
      },
      (error) => {
        setResult(`Failed to read connection state: ${error}`)
      }
    )
  }

  onMount(() => {
    void syncConnectionState()
  })

  function connect() {
    void runAction(
      'connect',
      commands.connectDevice(selectedMode),
      () => {
        connected = true
        setResult(`Connected to ${selectedMode}`)
      },
      'Connection failed'
    )
  }

  function disconnect() {
    void runAction(
      'disconnect',
      commands.disconnectDevice(),
      () => {
        connected = false
        setResult('Disconnected')
      },
      'Disconnect failed'
    )
  }

  function getBattery() {
    void runAction(
      'battery',
      commands.getBattery(),
      (level) => {
        setResult(level)
      },
      'Battery request failed'
    )
  }

  function sendName() {
    void runAction(
      'name',
      commands.sendName(name),
      () => {
        setResult('Name sent')
      },
      'Send name failed'
    )
  }

  function sendTemp() {
    void runAction(
      'temp',
      commands.sendTemp(city),
      () => {
        setResult('Weather sent')
      },
      'Send weather failed'
    )
  }

  async function sendBustime() {
    pending = 'bustime'

    try {
      let permissions = await checkPermissions()
      if (
        permissions.location === 'prompt' ||
        permissions.location === 'prompt-with-rationale'
      ) {
        permissions = await requestPermissions(['location'])
      }

      if (permissions.location !== 'granted') {
        setResult('Location permission is required')
        return
      }

      const pos = await getCurrentPosition()
      await tryCmd(
        commands.sendBustime(pos.coords.latitude, pos.coords.longitude)
      ).match(
        (data) => {
          setResult(data)
        },
        (error) => {
          setResult(`Bustime request failed: ${error}`)
        }
      )
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
      disabled={busy || connected}
      class="border rounded px-3 py-2"
    >
      <option value="RealDevice">Real Device (Bluetooth)</option>
      <option value="Simulator">Simulator (TCP)</option>
    </select>

    <Button
      variant="outline"
      onclick={connected ? disconnect : connect}
      disabled={busy}
    >
      {#if isPending('connect')}
        Connecting...
      {:else if isPending('disconnect')}
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
      disabled={busy || !name.trim()}
    >
      {isPending('name') ? 'Sending...' : 'Send Name'}
    </Button>
  </Field.Field>

  <Field.Field orientation="horizontal" class="justify-center mx-auto max-w-xs">
    <Field.Label for="city-input" class="sr-only">City</Field.Label>
    <Input id="city-input" placeholder="Enter a city..." bind:value={city} />
    <Button
      variant="outline"
      onclick={sendTemp}
      disabled={busy || !city.trim()}
    >
      {isPending('temp') ? 'Sending...' : 'Send Weather'}
    </Button>
  </Field.Field>

  <div class="flex justify-center gap-3">
    <Button variant="outline" onclick={getBattery} disabled={busy}>
      {isPending('battery') ? 'Checking...' : 'Device Battery'}
    </Button>

    <Button variant="outline" onclick={sendBustime} disabled={busy}>
      {isPending('bustime') ? 'Sending...' : 'Send Bustime'}
    </Button>
  </div>

  {#if result !== null}
    <p class="text-center text-sm">{result}</p>
  {/if}
</main>
