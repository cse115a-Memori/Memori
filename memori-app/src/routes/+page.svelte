<script lang="ts">
	import type { UnlistenFn } from '@tauri-apps/api/event'
  import { onDestroy, onMount } from 'svelte'
  import { taurpcContext } from '@/tauri/context'
  import { Button } from '$lib/components/ui/button/index.js'
  import * as Field from '$lib/components/ui/field/index.js'
  import { Input } from '$lib/components/ui/input/index.js'
  import { createTauRPCProxy, type InferCommandOutput, type Router } from '$lib/ipc'

  let name = $state('')
  let string = $state('')
  let res: number | string = $state('')
  let unlisten: UnlistenFn[] = $state([])
  let taurpc: ReturnType<typeof createTauRPCProxy>

  onMount(async () => {
    taurpc = createTauRPCProxy()
    // taurpc = taurpcContext.get()
  })

  const call_backend = async (e: Event) => {
    e.preventDefault()
    try {
      res = await taurpc.hello(name)
      console.log(res)
    } catch (error) {
      console.error(error)
    }
  }
  const get_battery = async (e: Event) => {
    e.preventDefault()
    try {
      res = await taurpc.get_battery()
      console.log(res)
    } catch (error) {
      console.error(error)
    }
  }
  const connect = async (e: Event) => {
    e.preventDefault()
    try {
      res = await taurpc.connect()
      console.log(res)
    } catch (error) {
      console.error(error)
    }
  }
  const send_string = async (e: Event) => {
    e.preventDefault()
    try {
      res = await taurpc.send_string(string)
      console.log(res)
    } catch (error) {
      console.error(error)
    }
  }
</script>

<main>
	<form class="mt-4" onsubmit={call_backend}>
		<Field.Field
			orientation="horizontal"
			class="justify-center mx-auto max-w-xs"
		>
			<Field.Label for="greet-input" class="sr-only">Name</Field.Label>

			<Input
				id="greet-input"
				placeholder="Enter a name..."
				bind:value={name}
			/>

			<Button type="submit" variant="outline">Greet</Button>
		</Field.Field>
	</form>

	<form class="mt-4" onsubmit={send_string}>
		<Field.Field
			orientation="horizontal"
			class="justify-center mx-auto max-w-xs"
		>
			<Field.Label for="greet-input" class="sr-only"
			>show on display</Field.Label>

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
	<form class="mt-4" onsubmit={connect}>
		<Field.Field>
			<Button type="submit" variant="outline">Connect to device</Button>
		</Field.Field>
	</form>
	{res}
</main>
