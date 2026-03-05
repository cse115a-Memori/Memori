<script lang="ts">
	import { Button } from '$lib/components/ui/button'

	type IntegrationId = 'github' | 'twitch' | 'google'

	type IntegrationCard = {
		id: IntegrationId
		label: string
		blurb: string
		connected: boolean
	}

	let {
		cards,
	}: {
		cards: IntegrationCard[]
	} = $props()
</script>

<div class="space-y-8">
	<div class="space-y-3">
		<p class="text-[0.78rem] uppercase tracking-[0.24em] text-foreground/48">Optional</p>
		<p class="text-[0.98rem] leading-7 text-foreground/70">
			These services belong here eventually. For now, it is enough to name them honestly and leave them quiet.
		</p>
	</div>

	<div class="space-y-5 border-t border-[rgba(63,53,43,0.12)] pt-5">
		{#each cards as card (card.id)}
			<div class="space-y-2">
				<div class="flex items-center justify-between gap-3">
					<p class="font-serif text-[1.35rem] text-foreground/84">{card.label}</p>
					<Button
						variant="outline"
						class="h-10 rounded-none border-[rgba(63,53,43,0.18)] bg-transparent px-3 text-foreground/60 shadow-none"
						disabled
					>
						{card.connected ? 'Detected' : 'Soon'}
					</Button>
				</div>
				<p class="text-sm leading-7 text-foreground/62">{card.blurb}</p>
			</div>
		{/each}
	</div>

	<p class="text-sm leading-7 text-foreground/56">
		You are not being hurried. The flow can finish cleanly now, and the rest can arrive later.
	</p>
</div>
