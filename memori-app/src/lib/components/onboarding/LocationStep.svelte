<script lang="ts">
	import type { Position } from '@tauri-apps/plugin-geolocation'
	import { Button } from '$lib/components/ui/button'
	import type { LocationStatus } from '$lib/features/prefs'

	type FeedbackTone = 'neutral' | 'success' | 'error'

	type Props = {
		locationStatus: LocationStatus
		lastKnownLocation: Position | null
		isRequesting: boolean
		actionMessage: string
		actionTone: FeedbackTone
		onRequest: () => void
	}

	let {
		locationStatus,
		lastKnownLocation,
		isRequesting,
		actionMessage,
		actionTone,
		onRequest,
	}: Props = $props()

	const isGranted = $derived(locationStatus === 'granted')
	const canRequest = $derived(
		locationStatus === 'prompt' || locationStatus === 'prompt-with-rationale'
	)
	const showCoordinates = $derived(
		locationStatus === 'granted' || lastKnownLocation !== null
	)
</script>

<div class="space-y-8">
	<div class="space-y-3">
		<p class="text-[0.78rem] uppercase tracking-[0.24em] text-foreground/48">Permission</p>
		<p class="text-[0.98rem] leading-7 text-foreground/70">
			Location is optional, but it lets Memori speak from where you actually are.
		</p>
	</div>

	<div class="space-y-5">
		<div class="space-y-2">
			<p class="text-[0.78rem] uppercase tracking-[0.24em] text-foreground/45">Status</p>
			<p class="font-serif text-[1.9rem] leading-8 text-foreground/88">
				{isGranted ? 'Granted.' : `${locationStatus}.`}
			</p>
		</div>

		<div class="border-t border-[rgba(63,53,43,0.12)] pt-5">
			{#if showCoordinates}
				<div class="space-y-2">
					<p class="text-[0.78rem] uppercase tracking-[0.24em] text-foreground/45">Last known place</p>
					<p class="font-serif text-[1.5rem] leading-8 text-foreground/84">
						{lastKnownLocation
							? `${lastKnownLocation.coords.latitude.toFixed(5)}, ${lastKnownLocation.coords.longitude.toFixed(5)}`
							: 'No sample yet'}
					</p>
					<p class="text-sm leading-7 text-foreground/62">
						{isGranted
							? 'Live updates are available whenever Memori asks for them.'
							: 'This is the most recent saved sample from an earlier session.'}
					</p>
				</div>
			{:else if locationStatus === 'not-available'}
				<p class="text-sm leading-7 text-foreground/62">
					Location services are not available on this platform. You may continue without them.
				</p>
			{:else if locationStatus === 'denied'}
				<p class="text-sm leading-7 text-foreground/62">
					Access is denied. You may continue and return later from system settings.
				</p>
			{:else}
				<p class="text-sm leading-7 text-foreground/62">
					Grant access once and local weather and transit can become part of the device's daily surface.
				</p>
			{/if}
		</div>

		{#if canRequest}
			<Button
				variant="outline"
				class="h-12 w-full rounded-none border border-[rgba(122,95,54,0.34)] bg-[rgba(122,95,54,0.06)] text-foreground shadow-none hover:bg-[rgba(122,95,54,0.12)]"
				disabled={isRequesting}
				onclick={onRequest}
			>
				{isRequesting ? 'Please wait' : 'Enable location'}
			</Button>
		{/if}
	</div>

	{#if actionMessage}
		<p class="border-l border-[rgba(122,95,54,0.34)] pl-4 text-sm leading-7 text-foreground/66">
			{#if actionTone === 'error'}
				Unable to update location: {actionMessage}
			{:else}
				{actionMessage}
			{/if}
		</p>
	{/if}
</div>
