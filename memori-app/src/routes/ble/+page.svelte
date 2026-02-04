<script lang="ts">
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

  import { createBlePageModel } from './ble-page-model.svelte'
  import {
    displayName,
    filterAndSortDevices,
    signalClass,
    signalStrength,
  } from './ble-utils'

  const { state, actions } = createBlePageModel()

  function filteredDevices() {
    return filterAndSortDevices(
      state.devices,
      bleStore.state.showAllDevices,
      bleStore.state.resolvedNames
    )
  }
</script>

<div class="mx-auto flex min-h-svh w-full max-w-4xl flex-col gap-6 px-4 py-8">
  <div class="flex flex-col gap-2">
    <div class="flex flex-wrap items-center gap-3">
      <h1 class="text-2xl font-semibold">BLE Control Panel</h1>
      <Badge variant={state.connected ? 'default' : 'secondary'}>
        {state.connected ? 'Connected' : 'Disconnected'}
      </Badge>
      <Badge variant={state.scanning ? 'default' : 'outline'}>
        {state.scanning ? 'Scanning' : 'Idle'}
      </Badge>
    </div>
  </div>

  <Card>
    <CardHeader>
      <CardTitle>Adapter & Permissions</CardTitle>
    </CardHeader>
    <CardContent class="flex flex-col gap-4">
      <div class="flex flex-wrap items-center gap-3">
        <Button variant="secondary" onclick={actions.checkState}>
          Check State
        </Button>
        <Badge variant="outline">{state.adapterState}</Badge>
      </div>
      <div class="flex flex-wrap items-center gap-3">
        <Button
          variant="secondary"
          onclick={() => actions.checkPermission(true)}
        >
          Check Permission (ask)
        </Button>
        <Button
          variant="secondary"
          onclick={() => actions.checkPermission(false)}
        >
          Check Permission (no ask)
        </Button>
        <Badge variant={state.permissionsGranted ? 'default' : 'secondary'}>
          {state.permissionsGranted ? 'Granted' : 'Not granted'}
        </Badge>
      </div>
    </CardContent>
  </Card>

  <Card>
    <CardHeader>
      <CardTitle>Scanning</CardTitle>
    </CardHeader>
    <CardContent class="flex flex-wrap items-center gap-3">
      {#if state.scanning}
        <Button variant="destructive" onclick={actions.stopScanDevices}>
          Stop Scan
          <Spinner class="ml-2" />
        </Button>
      {:else}
        <Button onclick={() => actions.startScanDevices(false)}>
          Start Scan
        </Button>
        <Button
          variant="secondary"
          onclick={() => actions.startScanDevices(true)}
        >
          Start Scan with iBeacons
        </Button>
      {/if}
    </CardContent>
  </Card>

  {#if state.connected}
    <Card>
      <CardHeader>
        <CardTitle>Connection</CardTitle>
      </CardHeader>
      <CardContent class="flex flex-col gap-4">
        <div class="flex flex-wrap items-center gap-3">
          <Button variant="destructive" onclick={actions.disconnectDevice}>
            Disconnect
          </Button>
          <Button variant="secondary" onclick={actions.testRust}>
            Test Rust communication
          </Button>
          {#if state.rustTest}
            <Badge>Rust test successful</Badge>
          {/if}
        </div>

        <Separator />

        <div class="grid gap-4 md:grid-cols-2">
          <div class="flex flex-col gap-3">
            <p class="text-sm font-medium">Service 1</p>
            <div class="flex items-center gap-2">
              <Input bind:value={state.sendData} placeholder="Send data" />
              <Button onclick={actions.sendPrimary}>
                Send
              </Button>
            </div>
            <div class="flex items-center gap-2">
              <Input bind:value={state.recvData} readonly />
              <Button variant="secondary" onclick={actions.readPrimary}>
                Read
              </Button>
            </div>
            <div class="flex items-center gap-2">
              <Input bind:value={state.notifyData} readonly />
              <Button variant="secondary" onclick={actions.toggleNotifyPrimary}>
                {state.notifyData ? 'Unsubscribe' : 'Subscribe'}
              </Button>
            </div>
          </div>

          <div class="flex flex-col gap-3">
            <p class="text-sm font-medium">Service 2</p>
            <div class="flex items-center gap-2">
              <Input bind:value={state.sendData2} placeholder="Send data" />
              <Button onclick={actions.sendSecondary}>
                Send
              </Button>
            </div>
            <div class="flex items-center gap-2">
              <Input bind:value={state.recvData2} readonly />
              <Button variant="secondary" onclick={actions.readSecondary}>
                Read
              </Button>
            </div>
            <div class="flex items-center gap-2">
              <Input bind:value={state.notifyData2} readonly />
              <Button variant="secondary" onclick={actions.toggleNotifySecondary}>
                {state.notifyData2 ? 'Unsubscribe' : 'Subscribe'}
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
            <Checkbox id="show-services" bind:checked={state.showServices} />
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
              onclick={() => actions.connectDevice(device)}
            >
              <div class="flex w-full flex-col gap-2">
                <div class="flex items-center justify-between gap-3">
                  <div class="flex min-w-0 flex-col">
                    <span class="truncate text-sm font-medium">
                      {displayName(device, bleStore.state.resolvedNames)}
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
                {#if state.showServices}
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
