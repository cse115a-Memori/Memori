<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import type { DeviceMode } from '$lib/tauri/bindings'
  import type { UnlistenFn } from '@tauri-apps/api/event'
  import { Button } from '$lib/components/ui/button/index.js'
  import * as Field from '$lib/components/ui/field/index.js'
  import { Input } from '$lib/components/ui/input/index.js'
  import { commands } from '@/tauri'
  import { goto } from '$app/navigation'
  import { load } from '@tauri-apps/plugin-store';
  import {login, getCurrentUser} from '$lib/services/auth'; 
  import { getCurrent, onOpenUrl } from '@tauri-apps/plugin-deep-link';
  import {
    checkPermissions,
    requestPermissions,
    getCurrentPosition,
    watchPosition,
  } from '@tauri-apps/plugin-geolocation';

  let errorMessage = $state('');
  let isTwitchLoading = $state(false);
  let isLoading = $derived(isGoogleLoading || isGithubLoading || isTwitchLoading);
  let error: string | null = null;
  let name = $state('')
  let string = $state('')
  let res: number | string | null = $state('')
  let unlisten: UnlistenFn[] = $state([])
  let location: string = $state('')
  let city: string = $state('')
  let currentUser = $state(await getCurrentUser())
  let token = $derived(currentUser?.accessToken)

  const startUrls = await getCurrent();

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

  const send_name = async (e: Event) => {
    e.preventDefault()
    try {
      res = await invoke('send_name', { name })
      console.log(res)
    } catch (error) {
      console.error(error)
    }
  }
  
  const send_temp = async (e: Event) => {
    e.preventDefault()
    try {
      res = await invoke('send_temp', { city })
      console.log(res)
    } catch (error) {
      console.error(error)
    }
  }
  const send_bustime = async (e: Event) => {
    e.preventDefault()
    let permissions = await checkPermissions();  
    if (  
      permissions.location === 'prompt' ||  
      permissions.location === 'prompt-with-rationale'  
    ) {  
      permissions = await requestPermissions(['location']);  
    }  
    if (permissions.location === 'granted') {  
      const pos = await getCurrentPosition();  
      
      // Send position to device  
      try {  
        res = await invoke('send_bustime', { 
          lat: pos.coords.latitude, 
          lon: pos.coords.longitude })  
        console.log('Bustime sent with position:', pos);
        console.log(res)
      } catch (error) {  
        console.error('Failed to send bustime:', error);  
      }  
  
    } 
  }
  const login_twitch = async (e: Event) => {
    e.preventDefault()
    try {
      errorMessage = '';
      isTwitchLoading = true;
      await login('twitch');
      
    } catch (error) {
      console.error('Twitch login failed:', error);
      errorMessage = 'Twitch log failed, please retry';
    } finally {
      isTwitchLoading = false;
    }
  }
  const send_twitch = async (e: Event) => {
    e.preventDefault()
    try {
      res = await invoke('send_twitch', {token})
      console.log(res)
    } catch (error) {
      console.error(error) 
    }
  }
</script>

<main>
  <form class="mt-4" onsubmit={send_name}>
    <Field.Field
      orientation="horizontal"
      class="justify-center mx-auto max-w-xs"
    >
      <Field.Label for="greet-input" class="sr-only">Name</Field.Label>

      <Input id="greet-input" placeholder="Enter a name..." bind:value={name} />

      <Button type="submit" variant="outline">Greet</Button>
    </Field.Field>
  </form>

  <form class="mt-4" onsubmit={send_temp}>
    <Field.Field
      orientation="horizontal"
      class="justify-center mx-auto max-w-xs"
    >
      <Field.Label for="greet-input" class="sr-only">Name</Field.Label>

      <Input id="greet-input" placeholder="Enter a city..." bind:value={city} />

      <Button type="submit" variant="outline">Send</Button>
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
  <form class="mt-4" onsubmit={login_twitch}>
    <Field.Field>
      <Button type="submit" variant="outline">Connect to twitch</Button>
    </Field.Field>
  </form>
  <form class="mt-4" onsubmit={send_twitch}>
    <Field.Field>
      <Button type="submit" variant="outline">Send twitch</Button>
    </Field.Field>
  </form>
  <form class="mt-4" onsubmit={send_bustime}>
    <Field.Field
      orientation="horizontal"
      class="justify-center mx-auto max-w-xs"
    >
      <Field.Label for="dropdown-select" class="sr-only"
        >Select Option</Field.Label
      >

      <select
        id="dropdown-select"
        bind:value={location}
        class="border rounded p-2"
      >
        <option value="" disabled selected>Select an option</option>
        <option value="1">Science Hill</option>
        <option value="2">Base of Campus (Barn)</option>
        <option value="3">Downtown Santa Cruz Metro Center</option>
      </select>

      <Button type="submit" variant="outline">Save Selection</Button>
    </Field.Field>
  </form>
  {res}
</main>
