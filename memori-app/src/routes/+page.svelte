<script lang="ts">
  import { appState } from '@/stores/app-store'
  import { Button } from '$lib/components/ui/button'
</script>

<main class="space-y-6 py-8">
  <header class="space-y-2">
    <h1 class="text-2xl font-semibold tracking-tight">Memori App</h1>
    <p class="text-sm text-muted-foreground">
      Use the routes below for auth, device controls, drag-and-drop, and
      location.
    </p>
  </header>

  <div class="flex flex-wrap gap-3">
    <Button variant="outline" href="/login">Login</Button>
    <Button variant="outline" href="/device">Device</Button>
    <Button variant="outline" href="/widgets">Widgets</Button>
    <Button variant="outline" href="/location">Location</Button>
  </div>

  <section class="space-y-2 rounded-md border p-4">
    <h2 class="text-sm font-medium tracking-tight">App Status</h2>

    <div class="space-y-1 text-sm">
      <p>Location: {appState.locationStatus}</p>
      <p>
        Last Known Location:
        {appState.lastKnownLocation
          ? `${appState.lastKnownLocation.coords.latitude.toFixed(6)}, ${appState.lastKnownLocation.coords.longitude.toFixed(6)}`
          : 'None'}
      </p>
      <p>Onboarding: {appState.onboarded ? 'Completed' : 'Pending'}</p>
      <p>Last Known Device: {appState.lastKnownDeviceId ?? 'None'}</p>
    </div>
  </section>
</main>
