<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import type { DeviceMode } from '$lib/tauri/bindings'
  import type { UnlistenFn } from '@tauri-apps/api/event'
  import { Button } from '$lib/components/ui/button/index.js'
  import * as Field from '$lib/components/ui/field/index.js'
  import { Input } from '$lib/components/ui/input/index.js'
  import { commands } from '@/tauri'

  let name = $state('')
  let string = $state('')
  let res: number | string | null = $state('')
  let unlisten: UnlistenFn[] = $state([])

  let isConnected = $state(false)
  let connecting = $state(false)
  let selectedMode: DeviceMode = $state('RealDevice')

  const get_battery = async (e: Event) => {
    e.preventDefault()
    try {
      const result = await commands.getBattery()
      if (result.status == 'ok') {
        res = result.data
      } else {
        res = result.error
      }
      console.log(res)
    } catch (error) {
      console.error(error)
      res = 'Unexpected invoke error'
    }
  }

  const connect = async (e: Event) => {
    e.preventDefault()
    connecting = true
    res = ''
    try {
      await invoke('connect_device', { mode: selectedMode })
      res = `connected to ${selectedMode}!`
      isConnected = true
      console.log(res)
    } catch (error) {
      console.error(error)
      res = `connection failed: ${error}`
    } finally {
      connecting = false
    }
  }

  const disconnect = async (e: Event) => {
    e.preventDefault()
    connecting = true
    res = ''
    try {
      await invoke('disconnect_device')
      res = 'Disconnected'
      isConnected = false
      console.log(res)
    } catch (error) {
      console.error(error)
      res = `disconnect failed: ${error}`
    } finally {
      connecting = false
    }
  }


  const send_string = async (e: Event) => {
    e.preventDefault()
    try {
      res = await invoke('hello', { name })
      console.log(res)
    } catch (error) {
      console.error(error)
    }
  }
</script>

<main>
  <form class="mt-4" onsubmit={send_string}>
    <Field.Field
      orientation="horizontal"
      class="justify-center mx-auto max-w-xs"
    >
      <Field.Label for="greet-input" class="sr-only">Name</Field.Label>

      <Input id="greet-input" placeholder="Enter a name..." bind:value={name} />

      <Button type="submit" variant="outline">Greet</Button>
    </Field.Field>
  </form>

  <form class="mt-4" onsubmit={send_string}>
    <Field.Field
      orientation="horizontal"
      class="justify-center mx-auto max-w-xs"
    >
      <Field.Label for="greet-input" class="sr-only"
        >show on display</Field.Label
      >

      <Input
        id="greet-input"
        placeholder="Enter a string!"
        bind:value={string}
      />

      <Button type="submit" variant="outline">display!</Button>
    </Field.Field>
  </form>

  <form class="mt-4" onsubmit={get_battery}>
    <Field.Field>
      <Button type="submit" variant="outline">Device Battery</Button>
    </Field.Field>
  </form>

  <form class="mt-4" onsubmit={isConnected ? disconnect : connect}>
    <Field.Field
      orientation="horizontal"
      class="justify-center mx-auto max-w-xs"
    >
      <Field.Label for="device-mode" class="sr-only">Device Mode</Field.Label>
      <select
        id="device-mode"
        bind:value={selectedMode}
        disabled={connecting || isConnected}
        class="border rounded px-3 py-2"
      >
        <option value="RealDevice">Real Device (Bluetooth)</option>
        <option value="Simulator">Simulator (TCP)</option>
      </select>
      <Button type="submit" variant="outline" disabled={connecting}>
        {#if connecting}
          {isConnected ? 'Disconnecting...' : 'Connecting...'}
        {:else}
          {isConnected ? 'Disconnect' : 'Connect'}
        {/if}
      </Button>
    </Field.Field>
  </form>

  {res}
</main>
