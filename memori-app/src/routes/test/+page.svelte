<script lang="ts">
	import { onMount } from 'svelte'
	import { prefsState } from '@/features/prefs/store.ts'
	import { Button } from '$lib/components/ui/button'

	let now = $state(new Date())

	const formattedTime = $derived(
		new Intl.DateTimeFormat(undefined, {
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit',
			timeZone: prefsState.systemOptions.timeZone ?? undefined,
			timeZoneName: 'short',
		}).format(now)
	)

	onMount(() => {
		const intervalId = setInterval(() => {
			now = new Date()
		}, 1000)

		return () => {
			clearInterval(intervalId)
		}
	})
</script>

<main class="space-y-6 py-8">
	<header class="space-y-2">
		<h1 class="text-2xl font-semibold tracking-tight">Clock Test</h1>
		<p class="text-sm text-muted-foreground">
			Timezone is auto-detected from the system and updates every second.
		</p>
	</header>

	<section class="space-y-2 rounded-md border p-4">
		<p class="text-xs text-muted-foreground">System Clock</p>
		<p class="text-3xl font-semibold tabular-nums">{formattedTime}</p>
		<p class="text-xs text-muted-foreground">
			Timezone: {prefsState.systemOptions.timeZone ?? 'Unavailable'}
		</p>
	</section>

	<div><Button variant="outline" href="/">Back Home</Button></div>
</main>
