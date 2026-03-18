<script lang="ts">
  import { goto } from '$app/navigation'
  import { Button } from '@/components/ui/button'
  import { connState, retryConnection } from '@/features/connection'
  import { prefsState } from '@/features/prefs/store'
  import { resetWidgets } from '@/features/widgets/widgets-store'

  async function resetOnboarding() {
    prefsState.onboarded = false
    await goto('/', { replaceState: true })
  }
  
let isRetrying = $state(false)
let dots = $state('')

async function handleRetry() {
  isRetrying = true
  const interval = setInterval(() => {
    dots = dots.length >= 3 ? '' : dots + '.'
  }, 400)
  await retryConnection()?.match(
    () => console.log("Connection successful"),
    (err) => console.log("Connection failed", err)
  )
  clearInterval(interval)
  isRetrying = false
}
</script>

<main class="mx-auto space-y-6 py-8">
  <header class="space-y-2">
    <h1 class="text-2xl font-semibold tracking-tight">Settings</h1>
    <p class="text-sm text-muted-foreground">
      Manage local app state and jump to the advanced testing surface when needed.
    </p>
  </header>

  <section class="space-y-2 rounded-2xl border bg-card p-4 shadow-sm">
    <h2 class="text-sm font-medium tracking-tight">Status</h2>
    <div class="space-y-1 text-sm text-muted-foreground">
      <p>Onboarding: {prefsState.onboarded ? 'Completed' : 'Pending'}</p>
      <p>Connection: {connState.isConnected ? 'Connected' : 'Disconnected'}</p>
      <p>Last Known Device: {prefsState.lastKnownDeviceId ?? 'None'}</p>
    </div>
  </section>

  <section class="grid gap-3 sm:grid-cols-2">
    <Button variant="outline" onclick={resetOnboarding}>Reset Onboarding</Button>
    <Button variant="outline" onclick={handleRetry} disabled={connState.isConnected || isRetrying}>
      {isRetrying ? `Retrying${dots}` : 'Retry Connection'}
    </Button>
    <Button variant="outline" onclick={() => (connState.deviceCode = '')}>
      Reset DeviceId
    </Button>
    <Button variant="outline" onclick={resetWidgets}>Reset Widgets</Button>
  </section>

  <div class="flex flex-wrap gap-2">
    <Button variant="ghost" href="/device">Open Device Controls</Button>
    <Button variant="ghost" href="/testing">Open Testing Tools</Button>
  </div>
</main>
