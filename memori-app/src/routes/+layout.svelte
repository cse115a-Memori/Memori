<script lang="ts">
	import { onMount } from 'svelte'
	import { Button } from '@/components/ui/button'
	import { startAuthStore } from '@/features/auth/store'
	import { startPrefsStore } from '@/features/prefs/store'
	import { startWidgetsStore } from '@/features/widgets/widgets-store'
	import { page } from '$app/state'
	import '../app.css'

	const { children } = $props()

	onMount(() => {
		void Promise.all([startPrefsStore(), startWidgetsStore(), startAuthStore()]).catch(
			error => {
				console.error('Failed to start stores:', error)
			}
		)
	})
</script>

<div class="min-h-dvh">
	<div class="mx-auto w-full max-w-screen-sm px-4 py-6">
		{@render navLinks('/', 'Home')}
		{@render navLinks('/login', 'Login')}
		{@render navLinks('/device', 'Device')}
		{@render navLinks('/location', 'Location')}
		{@render children?.()}
	</div>
</div>

{#snippet navLinks(route: string, name: string)}
	<Button
		variant="link"
		href={route}
		class={`${page.url.pathname === route ? 'font-bold' : ''} transition-all`}
	>
		{name}
	</Button>
{/snippet}
