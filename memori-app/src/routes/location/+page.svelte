<script lang="ts">
  import { onMount } from 'svelte'
  import { Button } from '@/components/ui/button'
  import {
    refreshLocationState,
    requestLocationState,
  } from '@/services/location-service'
  import { appState, startAppStore } from '@/stores/app-store'

  let errorMessage = $state('')
  let isRequesting = $state(false)
  let hasCheckedPermission = $state(false)
  const locationStatus = $derived(appState.locationStatus)
  const lastKnownLocation = $derived(appState.lastKnownLocation)
  const canEnableLocation = $derived(
    hasCheckedPermission &&
      (locationStatus === 'prompt' ||
        locationStatus === 'prompt-with-rationale')
  )

  onMount(() => {
    void (async () => {
      try {
        await startAppStore()
        await refreshLocationState()
      } catch (error) {
        errorMessage =
          typeof error === 'string'
            ? error
            : error instanceof Error
              ? error.message
              : String(error)
      } finally {
        hasCheckedPermission = true
      }
    })()
  })

  async function enableLocation() {
    isRequesting = true
    errorMessage = ''
    try {
      await requestLocationState()
    } catch (error) {
      errorMessage =
        typeof error === 'string'
          ? error
          : error instanceof Error
            ? error.message
            : String(error)
    } finally {
      isRequesting = false
    }
  }
</script>

<main class="space-y-4">
  <h1 class="text-2xl font-semibold">Location</h1>
  <p class="text-sm text-muted-foreground">
    Location testing page for permission state and coordinate sampling.
  </p>

  <section class="space-y-1 rounded-md border p-4 text-sm">
    <p>Status: {locationStatus}</p>

    {#if locationStatus === 'not-available'}
      <p class="text-muted-foreground">
        Location is not available on this platform.
      </p>
    {:else}
      {#if locationStatus === 'granted' || lastKnownLocation}
        <p>
          Lat: {lastKnownLocation?.coords.latitude?.toFixed(6) ?? 'None'}
        </p>
        <p>
          Long: {lastKnownLocation?.coords.longitude?.toFixed(6) ?? 'None'}
        </p>
      {:else}
        <p class="text-muted-foreground">No location sample available yet.</p>
      {/if}

      {#if locationStatus !== 'granted' && lastKnownLocation}
        <p class="text-amber-600">
          Showing last known location from a previous session. Enable location
          for live updates.
        </p>
      {/if}

      {#if locationStatus === 'denied'}
        <p class="text-amber-600">
          Location access is denied. Enable it in Settings > Privacy > Location.
        </p>
      {/if}

      {#if canEnableLocation}
        <Button
          variant="outline"
          onclick={enableLocation}
          disabled={isRequesting}
        >
          {isRequesting ? 'Requesting...' : 'Enable Location'}
        </Button>
      {/if}
    {/if}
  </section>

  {#if errorMessage}
    <p class="text-red-500 text-sm">{errorMessage}</p>
  {/if}
</main>
