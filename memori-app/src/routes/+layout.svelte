<script lang="ts">
  import { onMount } from 'svelte'
  import { Button } from '@/components/ui/button'
  import { startAuthStore } from '@/features/auth/store'
  import {
    connectDevice,
    connState,
    syncConnectionState,
  } from '@/features/connection'
  import { startGitHubStore } from '@/features/github'
  import { prefsState, startPrefsStore } from '@/features/prefs/store'
  import {
    resetWidgets,
    startWidgetsStore,
    type WidgetsState,
    widgetsState,
  } from '@/features/widgets/widgets-store'
  import { goto, onNavigate } from '$app/navigation'
  // import { refreshLocationState } from '@/features/prefs/service'
  import { page } from '$app/state'

  import '../app.css'
  import { LoaderCircle } from '@lucide/svelte'
  import { selectFlashPayload } from '@/features/widgets/flash'
  import { commands, tryCmd } from '@/tauri'

  const { children } = $props()
  const isOnboardingRoute = $derived(page.url.pathname === '/onboarding')
  let isReady = $state(false)

  async function resetOnboarding() {
    prefsState.onboarded = false
    await goto('/', { replaceState: true })
  }

  onNavigate((navigation) => {
    if (!document.startViewTransition) return

    return new Promise((resolve) => {
      document.startViewTransition(async () => {
        resolve()
        await navigation.complete
      })
    })
  })

  onMount(async () => {
    void Promise.all([
      startPrefsStore(),
      startWidgetsStore(),
      startAuthStore(),
      startGitHubStore(),
      syncConnectionState(),
      // refreshLocationState(),
    ])
      .catch((error) => {
        console.error('Failed to start stores:', error)
      })
      .finally(() => {
        isReady = true
      })

    // check if onboarded, if not goto(/onboard)
    if (connState.deviceCode !== '') {
      await connectDevice('RealDevice', connState.deviceCode).match(
        () => {
          connState.isConnected = true
        },
        (error) => {
          connState.isConnected = false
        }
      )
    }
  })

  const snapshot = $state.snapshot(widgetsState) as WidgetsState
  const payload = selectFlashPayload(snapshot)
  $inspect('widgetState from layout', payload, widgetsState)
</script>

{#if isReady}
  <div class="h-dvh">
    {#if isOnboardingRoute}
      {@render children?.()}
    {:else}
      <div class="mx-auto w-full max-w-screen-sm px-4 py-6">
        <div class="mb-4 flex flex-wrap items-center gap-2">
          {@render navLinks('/', 'Home')}
          {@render navLinks('/device', 'Device')}
          <!-- {@render navLinks('/location', 'Location')} -->
          {@render navLinks('/onboarding', 'Onboarding')}
          {@render navLinks('/testing', 'Testing')}
          <Button variant="outline" class="ml-auto" onclick={resetOnboarding}>
            Reset Onboarding
          </Button>
          <Button
            variant="outline"
            class="ml-auto"
            onclick={syncConnectionState}
          >
            Check Connection {connState.isConnected
              ? 'Connected'
              : 'Disconnected'}
          </Button>
          <Button
            variant="outline"
            class="ml-auto"
            onclick={() => (connState.deviceCode = '')}
          >
            Reset DeviceId
          </Button>
          <Button variant="outline" class="ml-auto" onclick={resetWidgets}>
            Reset Widgets
          </Button>
        </div>
        {@render children?.()}
      </div>
    {/if}
  </div>
{:else}
  <div class="flex min-h-dvh items-center justify-center">
    <LoaderCircle class="animate-spin" />
  </div>
{/if}

{#snippet navLinks(route: string, name: string)}
  <Button
    variant="link"
    href={route}
    class={`${page.url.pathname === route ? 'font-bold' : ''} transition-all`}
  >
    {name}
  </Button>
{/snippet}
