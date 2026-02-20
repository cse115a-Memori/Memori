<script lang="ts">
  import { onMount } from 'svelte'
  import type { UserInfo } from '@/tauri'
  import { commands, tryCmd } from '@/tauri'
  import { Button } from '$lib/components/ui/button/index.js'
  import { getUser, login, logout } from '$lib/services/auth-service'

  type PendingAction = 'hydrating' | 'login' | 'send' | 'logout'

  let errorMessage = $state('')
  let statusMessage = $state('')
  let pendingAction = $state<PendingAction | null>('hydrating')

  let currentUser = $state<UserInfo | null>(null)
  const isBusy = $derived(pendingAction !== null)
  const accessToken = $derived(currentUser?.accessToken ?? null)

  onMount(() => {
    void hydrateCurrentUser()
  })

  async function hydrateCurrentUser() {
    await getUser('twitch').match(
      (user) => {
        currentUser = user
      },
      (error) => {
        errorMessage = `Failed to restore session: ${error}`
      }
    )
    pendingAction = null
  }

  async function loginTwitch() {
    errorMessage = ''
    statusMessage = ''
    pendingAction = 'login'
    await login('twitch').match(
      (user) => {
        currentUser = user
        statusMessage = 'Logged in with Twitch'
      },
      (error) => {
        errorMessage = `Twitch login failed: ${error}`
      }
    )
    pendingAction = null
  }

  async function sendTwitch() {
    if (!accessToken) {
      statusMessage = 'Missing OAuth access token'
      return
    }

    pendingAction = 'send'
    await tryCmd(commands.sendTwitch(accessToken)).match(
      (data) => {
        statusMessage = data
      },
      (error) => {
        statusMessage = `Send twitch failed: ${error}`
      }
    )
    pendingAction = null
  }

  async function logoutTwitch() {
    pendingAction = 'logout'
    statusMessage = ''
    errorMessage = ''
    await logout('twitch').match(
      () => {
        currentUser = null
        statusMessage = 'Logged out'
      },
      (error) => {
        errorMessage = `Logout failed: ${error}`
      }
    )
    pendingAction = null
  }
</script>

<main>
  {#if pendingAction === 'hydrating'}
    <p class="mt-4 text-center text-sm text-muted-foreground">
      Loading your session...
    </p>
  {/if}

  {#if errorMessage}
    <p class="mt-4 text-center text-sm text-red-500">{errorMessage}</p>
  {/if}

  {#if currentUser}
    <div class="mt-4 text-center text-sm">
      <p>Logged in as {currentUser.name} ({currentUser.provider})</p>
      <p class="text-muted-foreground">{currentUser.email}</p>
    </div>
  {/if}

  <div class="mt-4">
    <Button variant="outline" onclick={loginTwitch} disabled={isBusy}>
      Connect to twitch
    </Button>
  </div>

  <div class="mt-4">
    <Button
      variant="outline"
      onclick={sendTwitch}
      disabled={isBusy || !accessToken}
    >
      Send twitch
    </Button>
  </div>

  <div class="mt-4">
    <Button
      variant="outline"
      onclick={logoutTwitch}
      disabled={isBusy || !currentUser}
    >
      Logout
    </Button>
  </div>

  <p class="mt-4 text-center">
    <Button variant="link" href="/">Back Home</Button>
  </p>

  {#if statusMessage}
    <p class="mt-4 text-center">{statusMessage}</p>
  {/if}
</main>
