<script lang="ts">
  import { onMount } from 'svelte'
  import type { UserInfo } from '@/tauri'
  import { commands, tryCmd } from '@/tauri'
  import { Button } from '$lib/components/ui/button/index.js'
  import * as Field from '$lib/components/ui/field/index.js'
  import { getUser, login, logout } from '$lib/services/auth-service'

  type PendingAction = 'hydrating' | 'login' | 'send' | 'logout'

  let errorMessage = $state('')
  let statusMessage = $state('')
  let pendingAction = $state<PendingAction | null>('hydrating')
  let isLoading = $derived(pendingAction !== null)

  let currentUser = $state<UserInfo | null>(null)
  let token = $derived(currentUser?.accessToken)

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

  const loginTwitch = async (e: Event) => {
    e.preventDefault()
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

  const sendTwitch = async (e: Event) => {
    e.preventDefault()
    if (!token) {
      statusMessage = 'Missing OAuth access token'
      return
    }

    pendingAction = 'send'
    await tryCmd(commands.sendTwitch(token)).match(
      (data) => {
        statusMessage = data
      },
      (error) => {
        statusMessage = `Send twitch failed: ${error}`
      }
    )
    pendingAction = null
  }

  const logoutTwitch = async (e: Event) => {
    e.preventDefault()
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

  <fieldset disabled={isLoading} class="contents">
    <form class="mt-4" onsubmit={loginTwitch}>
      <Field.Field>
        <Button type="submit" variant="outline">Connect to twitch</Button>
      </Field.Field>
    </form>

    <form class="mt-4" onsubmit={sendTwitch}>
      <Field.Field>
        <Button type="submit" variant="outline" disabled={!token}
          >Send twitch</Button
        >
      </Field.Field>
    </form>

    <form class="mt-4" onsubmit={logoutTwitch}>
      <Field.Field>
        <Button type="submit" variant="outline" disabled={!currentUser}
          >Logout</Button
        >
      </Field.Field>
    </form>
  </fieldset>

  <p class="mt-4 text-center">
    <Button variant="link" href="/">Back Home</Button>
  </p>

  {#if statusMessage}
    <p class="mt-4 text-center">{statusMessage}</p>
  {/if}
</main>
