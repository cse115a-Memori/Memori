<script lang="ts">
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
  import { saveAll } from '@tauri-store/svelte'
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'

  import { Badge } from '$lib/components/ui/badge/index'
  import { Button } from '$lib/components/ui/button/index'
  import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
  } from '$lib/components/ui/card/index'
  import { Checkbox } from '$lib/components/ui/checkbox/index'
  import { Input } from '$lib/components/ui/input/index'
  import { Label } from '$lib/components/ui/label/index'
  import { Separator } from '$lib/components/ui/separator/index'
  import { Spinner } from '$lib/components/ui/spinner/index'
  import { store as bleStore } from '$lib/stores/ble.svelte'

  let devices = $state<BleDevice[]>([])
  let connected = $state(false)
  let scanning = $state(false)

  onMount(async () => {
    await getConnectionUpdates((state) => (connected = state))
    await getScanningUpdates((state) => {
      console.log('Scanning:', state)
      scanning = state
      if (!state) {
        resolveMissingNames()
      }
    })
  })

  const SERVICE_UUID = 'A07498CA-AD5B-474E-940D-16F1FBE7E8CD'
  const SERVICE2_UUID = 'A07498CA-AD5B-474E-940D-16F1FBE7E8CE'
  const CHARACTERISTIC_UUID = '51FF12BB-3ED8-46E5-B4F9-D64E2FEC021B'
  const GAP_SERVICE_UUID = '00001800-0000-1000-8000-00805F9B34FB'
  const DEVICE_NAME_UUID = '00002A00-0000-1000-8000-00805F9B34FB'

  let sendData = $state('')
  let recvData = $state('')
  let rustTest = $state(false)

  let sendData2 = $state('')
  let recvData2 = $state('')

  let resolvingNames = $state(false)
  let skipInitialSave = true

  $effect(() => {
    void bleStore.state.showAllDevices
    if (skipInitialSave) {
      skipInitialSave = false
      return
    }
    void saveAll()
  })

  function displayName(device: BleDevice) {
    return device.name || bleStore.state.resolvedNames[device.address] || ''
  }

  function isRecognizableName(name: string) {
    const trimmed = name.trim()
    if (!trimmed) return false

    const lower = trimmed.toLowerCase()
    if (lower === 'unknown' || lower === 'n/a') return false

    const letters = (trimmed.match(/[a-z]/gi) || []).length
    const vowels = (trimmed.match(/[aeiou]/gi) || []).length
    const digits = (trimmed.match(/\d/g) || []).length
    const separators = (trimmed.match(/[-_:\s]/g) || []).length
    const hasSpace = /\s/.test(trimmed)

    if (letters < 3 || vowels === 0) return false
    if (digits > 0 && digits + separators >= trimmed.length - 1) return false
    if (digits >= letters * 2) return false
    if (/^[0-9a-f:\-]+$/i.test(trimmed) && letters <= 2) return false
    if (/[0-9a-f]{6,}/i.test(trimmed)) return false
    if (/^[0-9a-f]{4,}$/i.test(trimmed)) return false
    if (!hasSpace && trimmed.length >= 8 && vowels <= 1) return false

    return true
  }

  function filteredDevices() {
    const filtered = bleStore.state.showAllDevices
      ? devices
      : devices.filter((device) => isRecognizableName(displayName(device)))
    return filtered.slice().sort((a, b) => b.rssi - a.rssi)
  }

  function signalStrength(rssi: number) {
    if (rssi >= -55) return 'Excellent'
    if (rssi >= -65) return 'Strong'
    if (rssi >= -75) return 'Good'
    if (rssi >= -85) return 'Fair'
    return 'Weak'
  }

  function signalClass(rssi: number) {
    if (rssi >= -55) return 'bg-emerald-300'
    if (rssi >= -65) return 'bg-emerald-200'
    if (rssi >= -75) return 'bg-yellow-200'
    if (rssi >= -85) return 'bg-orange-200'
    return 'bg-red-200'
  }

  async function resolveMissingNames() {
    if (resolvingNames || connected || scanning) return
    resolvingNames = true

    let didUpdate = false

    for (const device of devices) {
      if (device.name || bleStore.state.resolvedNames[device.address]) continue

      try {
        await connect(device.address, () => {})
        const name = await readString(DEVICE_NAME_UUID, GAP_SERVICE_UUID)
        if (name) {
          bleStore.state.resolvedNames[device.address] = name
          didUpdate = true
        }
      } catch (e) {
        console.warn('Failed to resolve device name', device.address, e)
      } finally {
        try {
          await disconnect()
        } catch (e) {
          console.warn('Failed to disconnect after name lookup', e)
        }
      }
    }

    if (didUpdate) {
      await saveAll()
    }
    resolvingNames = false
  }

  let notifyData = $state('')
  async function subscribe() {
    if (notifyData) {
      unsubscribe(CHARACTERISTIC_UUID)
      notifyData = ''
    } else {
      subscribeString(
        CHARACTERISTIC_UUID,
        (data: string) => (notifyData = data)
      )
    }
  }

  let notifyData2 = $state('')
  async function subscribe2() {
    if (notifyData2) {
      unsubscribe(CHARACTERISTIC_UUID)
      notifyData2 = ''
    } else {
      subscribeString(
        CHARACTERISTIC_UUID,
        (data: string) => (notifyData2 = data)
      )
    }
  }

  async function test() {
    try {
      const resp = await invoke<boolean>('test')
      rustTest = resp
    } catch (e) {
      console.error(e)
    }
  }

  let showServices = $state(false)
  let adapterState = $state<AdapterState>('Unknown')
  let permissionsGranted = $state(false)

  async function checkState() {
    adapterState = await getAdapterState()
  }

  async function checkPermission(askIfDenied = true) {
    permissionsGranted = await checkPermissions(askIfDenied)
  }

  async function handleStartScan(withIBeacons = false) {
    startScan(
      (dev: BleDevice[]) => {
        devices = dev
      },
      10000,
      withIBeacons
    )
  }

  async function handleConnect(device: BleDevice) {
    await connect(device.address, () => console.log('disconnected'))
    console.log('connect command returned')
  }
</script>

<div class="mx-auto flex min-h-svh w-full max-w-4xl flex-col gap-6 px-4 py-8">
  <div class="flex flex-col gap-2">
    <div class="flex flex-wrap items-center gap-3">
      <h1 class="text-2xl font-semibold">BLE Control Panel</h1>
      <Badge variant={connected ? 'default' : 'secondary'}>
        {connected ? 'Connected' : 'Disconnected'}
      </Badge>
      <Badge variant={scanning ? 'default' : 'outline'}>
        {scanning ? 'Scanning' : 'Idle'}
      </Badge>
    </div>
  </div>

  <!-- <Card>
    <CardHeader>
      <CardTitle>Adapter & Permissions</CardTitle>
    </CardHeader>
    <CardContent class="flex flex-col gap-4">
      <div class="flex flex-wrap items-center gap-3">
        <Button variant="secondary" onclick={checkState}>Check State</Button>
        <Badge variant="outline">{adapterState}</Badge>
      </div>
      <div class="flex flex-wrap items-center gap-3">
        <Button variant="secondary" onclick={() => checkPermission(true)}>
          Check Permission (ask)
        </Button>
        <Button variant="secondary" onclick={() => checkPermission(false)}>
          Check Permission (no ask)
        </Button>
        <Badge variant={permissionsGranted ? 'default' : 'secondary'}>
          {permissionsGranted ? 'Granted' : 'Not granted'}
        </Badge>
      </div>
    </CardContent>
  </Card> -->

  <Card>
    <CardHeader>
      <CardTitle>Scanning</CardTitle>
    </CardHeader>
    <CardContent class="flex flex-wrap items-center gap-3">
      {#if scanning}
        <Button variant="destructive" onclick={stopScan}>
          Stop Scan
          <Spinner class="ml-2" />
        </Button>
      {:else}
        <Button onclick={() => handleStartScan(false)}>Start Scan</Button>
        <Button variant="secondary" onclick={() => handleStartScan(true)}>
          Start Scan with iBeacons
        </Button>
      {/if}
    </CardContent>
  </Card>

  {#if connected}
    <Card>
      <CardHeader>
        <CardTitle>Connection</CardTitle>
      </CardHeader>
      <CardContent class="flex flex-col gap-4">
        <div class="flex flex-wrap items-center gap-3">
          <Button variant="destructive" onclick={disconnect}>Disconnect</Button>
          <Button variant="secondary" onclick={test}>
            Test Rust communication
          </Button>
          {#if rustTest}
            <Badge>Rust test successful</Badge>
          {/if}
        </div>

        <Separator />

        <div class="grid gap-4 md:grid-cols-2">
          <div class="flex flex-col gap-3">
            <p class="text-sm font-medium">Service 1</p>
            <div class="flex items-center gap-2">
              <Input bind:value={sendData} placeholder="Send data" />
              <Button
                onclick={() =>
                  sendString(
                    CHARACTERISTIC_UUID,
                    sendData,
                    'withResponse',
                    SERVICE_UUID
                  )}
              >
                Send
              </Button>
            </div>
            <div class="flex items-center gap-2">
              <Input bind:value={recvData} readonly />
              <Button
                variant="secondary"
                onclick={async () =>
                  (recvData = await readString(
                    CHARACTERISTIC_UUID,
                    SERVICE_UUID
                  ))}
              >
                Read
              </Button>
            </div>
            <div class="flex items-center gap-2">
              <Input bind:value={notifyData} readonly />
              <Button variant="secondary" onclick={subscribe}>
                {notifyData ? 'Unsubscribe' : 'Subscribe'}
              </Button>
            </div>
          </div>

          <div class="flex flex-col gap-3">
            <p class="text-sm font-medium">Service 2</p>
            <div class="flex items-center gap-2">
              <Input bind:value={sendData2} placeholder="Send data" />
              <Button
                onclick={() =>
                  sendString(
                    CHARACTERISTIC_UUID,
                    sendData2,
                    'withResponse',
                    SERVICE2_UUID
                  )}
              >
                Send
              </Button>
            </div>
            <div class="flex items-center gap-2">
              <Input bind:value={recvData2} readonly />
              <Button
                variant="secondary"
                onclick={async () =>
                  (recvData2 = await readString(
                    CHARACTERISTIC_UUID,
                    SERVICE2_UUID
                  ))}
              >
                Read
              </Button>
            </div>
            <div class="flex items-center gap-2">
              <Input bind:value={notifyData2} readonly />
              <Button variant="secondary" onclick={subscribe2}>
                {notifyData2 ? 'Unsubscribe' : 'Subscribe'}
              </Button>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  {:else}
    <Card>
      <CardHeader class="flex flex-col gap-3">
        <CardTitle>Devices</CardTitle>
        <div class="flex flex-wrap items-center gap-4">
          <div class="flex items-center gap-2">
            <Checkbox id="show-services" bind:checked={showServices} />
            <Label for="show-services">Show device details</Label>
          </div>
          <div class="flex items-center gap-2">
            <Checkbox
              id="show-all-devices"
              bind:checked={bleStore.state.showAllDevices}
            />
            <Label for="show-all-devices">Show all devices</Label>
          </div>
        </div>
      </CardHeader>
      <CardContent class="flex flex-col gap-3">
        {#if filteredDevices().length === 0}
          <p class="text-muted-foreground text-sm">
            No devices yet. Start a scan to populate the list.
          </p>
        {:else}
          {#each filteredDevices() as device (device.address)}
            <Button
              variant="outline"
              class="h-auto justify-start px-4 py-3 text-left"
              onclick={() => handleConnect(device)}
            >
              <div class="flex w-full flex-col gap-2">
                <div class="flex items-center justify-between gap-3">
                  <div class="flex min-w-0 flex-col">
                    <span class="truncate text-sm font-medium">
                      {displayName(device)}
                    </span>
                    <span class="truncate text-muted-foreground text-xs">
                      {device.address}
                    </span>
                  </div>
                  <Badge
                    variant="outline"
                    class={`shrink-0 ${signalClass(device.rssi)}`}
                  >
                    {signalStrength(device.rssi)}
                  </Badge>
                </div>
                {#if showServices}
                  <div class="text-muted-foreground text-xs">
                    {device.isConnected ? 'Connected' : 'Not connected'} •
                    {device.isBonded ? 'Bonded' : 'Not bonded'}
                  </div>
                {/if}
              </div>
            </Button>
          {/each}
        {/if}
      </CardContent>
    </Card>
  {/if}
</div>
