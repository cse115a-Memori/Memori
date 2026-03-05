<script lang="ts">
	import { Button } from '$lib/components/ui/button'
	import { Input } from '$lib/components/ui/input'
	import * as NativeSelect from '$lib/components/ui/native-select'
	import type { DeviceMode } from '$lib/tauri'

	type FeedbackTone = 'neutral' | 'success' | 'error'

	type Props = {
		deviceMode: DeviceMode
		pairingCode: string
		isBusy: boolean
		isConnected: boolean
		actionMessage: string
		actionTone: FeedbackTone
		onConnect: () => void
		onDisconnect: () => void
	}

	let {
		deviceMode = $bindable('RealDevice'),
		pairingCode = $bindable(''),
		isBusy,
		isConnected,
		actionMessage,
		actionTone,
		onConnect,
		onDisconnect,
	}: Props = $props()
</script>

<div class="space-y-8">
	<div class="space-y-3">
		<p class="text-[0.78rem] uppercase tracking-[0.24em] text-foreground/48">Connection</p>
		<p class="text-[0.98rem] leading-7 text-foreground/70">
			Choose the path, then pair once. Finishing onboarding still depends on a living connection.
		</p>
	</div>

	<div class="space-y-6">
		<label class="block space-y-3" for="device-mode">
			<span class="text-[0.92rem] leading-7 text-foreground/74">Connection mode</span>
			<NativeSelect.Root
				id="device-mode"
				bind:value={deviceMode}
				disabled={isBusy || isConnected}
				class="h-12 w-full rounded-none border-x-0 border-b border-t-0 border-[rgba(63,53,43,0.22)] bg-transparent px-0 text-base shadow-none focus-visible:border-[rgba(122,95,54,0.45)] focus-visible:ring-0"
			>
				<NativeSelect.Option value="RealDevice">Real Device (Bluetooth)</NativeSelect.Option>
				<NativeSelect.Option value="Simulator">Simulator (TCP)</NativeSelect.Option>
			</NativeSelect.Root>
		</label>

		<div class="space-y-2">
			<p class="text-[0.78rem] uppercase tracking-[0.24em] text-foreground/45">Status</p>
			<p class="font-serif text-[1.9rem] leading-8 text-foreground/88">
				{isConnected ? 'Connected.' : isBusy ? 'Acknowledging...' : 'Waiting.'}
			</p>
			<p class="text-sm leading-7 text-foreground/62">
				{isConnected
					? 'The device is ready. You may continue or disconnect and begin again.'
					: 'Keep the device awake and nearby before you begin.'}
			</p>
		</div>

		<Button
			class="h-12 w-full rounded-none border border-[rgba(122,95,54,0.34)] bg-[rgba(122,95,54,0.08)] text-foreground shadow-none hover:bg-[rgba(122,95,54,0.14)]"
			disabled={isBusy}
			onclick={isConnected ? onDisconnect : onConnect}
		>
			{#if isBusy}
				Please wait
			{:else if isConnected}
				Disconnect
			{:else}
				Begin connection
			{/if}
		</Button>
	</div>

	<div class="space-y-3 border-t border-[rgba(63,53,43,0.12)] pt-6">
		<p class="text-[0.78rem] uppercase tracking-[0.24em] text-foreground/45">Pair by code</p>
		<p class="text-sm leading-7 text-foreground/62">
			This alternate path has a place here, but its underlying logic is not yet ready.
		</p>
		<div class="flex gap-3">
			<Input
				bind:value={pairingCode}
				placeholder="Enter 6-digit code"
				class="h-12 rounded-none border-x-0 border-b border-t-0 border-[rgba(63,53,43,0.18)] bg-transparent px-0 shadow-none focus-visible:border-[rgba(122,95,54,0.45)] focus-visible:ring-0"
				disabled
			/>
			<Button
				variant="outline"
				class="h-12 rounded-none border-[rgba(63,53,43,0.18)] bg-transparent shadow-none"
				disabled
			>
				Soon
			</Button>
		</div>
	</div>

	{#if actionMessage}
		<p class="border-l border-[rgba(122,95,54,0.34)] pl-4 text-sm leading-7 text-foreground/66">
			{#if actionTone === 'error'}
				Unable to continue: {actionMessage}
			{:else}
				{actionMessage}
			{/if}
		</p>
	{/if}
</div>
