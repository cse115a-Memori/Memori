<script lang="ts">
	import { prefsState } from '@/features/prefs/store'
	import { Button } from '$lib/components/ui/button'
</script>

<main class="space-y-6 py-8 mx-auto">
	<header class="space-y-2">
		<h1 class="text-2xl font-semibold tracking-tight">Memori App</h1>
		<p class="text-sm text-muted-foreground">
			Use the routes below for auth, device controls, drag-and-drop, location, and
			testing.
		</p>
	</header>

	<section class="space-y-2 rounded-md border p-4">
		<h2 class="text-sm font-medium tracking-tight">App Status</h2>

		<div class="space-y-1 text-sm">
			<p>Location: {prefsState.locationStatus}</p>
			<p>
				Last Known Location:
				{prefsState.lastKnownLocation
          ? `${prefsState.lastKnownLocation.coords.latitude.toFixed(6)}, ${prefsState.lastKnownLocation.coords.longitude.toFixed(6)}`
          : 'None'}
			</p>
			<p>Onboarding: {prefsState.onboarded ? 'Completed' : 'Pending'}</p>
			<p>Last Known Device: {prefsState.lastKnownDeviceId ?? 'None'}</p>
		</div>
	</section>
	<!--
	<section class="space-y-4">
		<h1 class="text-2xl font-semibold">Location</h1>
		<p class="text-sm text-muted-foreground">
			Location testing page for permission state and coordinate sampling.
		</p>

		<section class="space-y-1 rounded-md border p-4 text-sm">
			<p>Status: {locationStatus}</p>

			{#if locationStatus === 'not-available'}
				<p class="text-muted-foreground">Location is not available on this platform.</p>
			{:else}
				{#if locationStatus === 'granted' || lastKnownPos}
					<p>Lat: {lastKnownPos?.coords.latitude?.toFixed(6) ?? 'None'}</p>
					<p>Long: {lastKnownPos?.coords.longitude?.toFixed(6) ?? 'None'}</p>
				{:else}
					<p class="text-muted-foreground">No location sample available yet.</p>
				{/if}

				{#if locationStatus !== 'granted' && lastKnownPos}
					<p class="text-amber-600">
						Showing last known location from a previous session. Enable location for
						live updates.
					</p>
				{/if}

				{#if locationStatus === 'denied'}
					<p class="text-amber-600">
						Location access is denied. Enable it in Settings > Privacy > Location.
					</p>
				{/if}

				{#if canEnableLocation}
					<Button variant="outline" onclick={enableLocation} disabled={isRequesting}>
						{isRequesting ? 'Requesting...' : 'Enable Location'}
					</Button>
				{/if}
			{/if}
		</section>

		{#if errMsg}
			<p class="text-red-500 text-sm">{errMsg}</p>
		{/if}
	</section> -->
</main>
