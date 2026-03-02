<script lang="ts">
	import { onMount } from 'svelte'
	import { getUser, login, logout } from '@/features/auth/service'
	import type { UserInfo } from '@/tauri'
	import { commands, tryCmd } from '@/tauri'
	import { Button } from '$lib/components/ui/button/index.js'

	type PendingAction = 'hydrating' | 'login' | 'send' | 'logout'

	let errMsg = $state('')
	let statusMsg = $state('')
	let pendingOp = $state<PendingAction | null>('hydrating')

	let user = $state<UserInfo | null>(null)
	const isBusy = $derived(pendingOp !== null)
	const accessToken = $derived(user?.accessToken ?? null)

	onMount(() => {
		void hydrateCurrentUser()
	})

	async function hydrateCurrentUser() {
		await getUser('twitch').match(
			nextUser => {
				user = nextUser
			},
			error => {
				errMsg = `Failed to restore session: ${error}`
			}
		)
		pendingOp = null
	}

	async function loginTwitch() {
		errMsg = ''
		statusMsg = ''
		pendingOp = 'login'
		await login('twitch').match(
			nextUser => {
				user = nextUser
				statusMsg = 'Logged in with Twitch'
			},
			error => {
				errMsg = `Twitch login failed: ${error}`
			}
		)
		pendingOp = null
	}

	async function sendTwitch() {
		if (!accessToken) {
			statusMsg = 'Missing OAuth access token'
			return
		}

		pendingOp = 'send'
		await tryCmd(commands.sendTwitch(accessToken)).match(
			data => {
				statusMsg = data ?? 'Twitch sent'
			},
			error => {
				statusMsg = `Send twitch failed: ${error}`
			}
		)
		pendingOp = null
	}

	async function logoutTwitch() {
		pendingOp = 'logout'
		statusMsg = ''
		errMsg = ''
		await logout('twitch').match(
			() => {
				user = null
				statusMsg = 'Logged out'
			},
			error => {
				errMsg = `Logout failed: ${error}`
			}
		)
		pendingOp = null
	}
</script>

<main>
	{#if pendingOp === 'hydrating'}
		<p class="mt-4 text-center text-sm text-muted-foreground">
			Loading your session...
		</p>
	{/if}

	{#if errMsg}
		<p class="mt-4 text-center text-sm text-red-500">{errMsg}</p>
	{/if}

	{#if user}
		<div class="mt-4 text-center text-sm">
			<p>Logged in as {user.name} ({user.provider})</p>
			<p class="text-muted-foreground">{user.email}</p>
		</div>
	{/if}

	<div class="mt-4">
		<Button variant="outline" onclick={loginTwitch} disabled={isBusy}>
			Connect to twitch
		</Button>
	</div>

	<div class="mt-4">
		<Button variant="outline" onclick={sendTwitch} disabled={isBusy || !accessToken}>
			Send twitch
		</Button>
	</div>

	<div class="mt-4">
		<Button variant="outline" onclick={logoutTwitch} disabled={isBusy || !user}>
			Logout
		</Button>
	</div>

	<p class="mt-4 text-center"><Button variant="link" href="/">Back Home</Button></p>

	{#if statusMsg}
		<p class="mt-4 text-center">{statusMsg}</p>
	{/if}
</main>
