<script lang="ts">
	import * as Carousel from '$lib/components/ui/carousel/index.js'
	import { Button } from '$lib/components/ui/button/index.js'

	type Step = {
		id: string
		title: string
		body: string
		image?: string
	}

	const steps: Step[] = [
		{
			id: 'welcome',
			title: 'Memori',
			body: 'Memento Mori &ndash; Remember you must die.',
		},
		{
			id: 'name',
			title: 'Name',
			body: "What's your name?",
		},
		{
			id: 'ble',
			title: 'Find Memori Device',
			body: "What's your pairing code?",
		},
	]

	let current = $state(0)
	let api = $state<any>(null)

	const isFirst = $derived(current === 0)
	const isLast = $derived(current === steps.length - 1)

	function handleApi(nextApi: any) {
		api = nextApi

		current = api.selectedScrollSnap()

		api.on('select', () => {
			current = api.selectedScrollSnap()
		})
	}

	function next() {
		api?.scrollNext()
	}

	function prev() {
		api?.scrollPrev()
	}

	function goTo(index: number) {
		api?.scrollTo(index)
	}

	function finish() {
		// e.g. localStorage, cookie, store, then goto("/app")
		console.log('onboarding complete')
	}
</script>

<div class="flex min-h-dvh flex-col bg-background">
	<div class="flex-1">
		<Carousel.Root
			class="h-full"
			opts={{
        align: "start",
        loop: false,
        dragFree: false
      }}
			setApi={handleApi}
		>
			<Carousel.Content class="ml-0 h-full">
				{#each steps as step (step.id)}
					<Carousel.Item class="pl-0 basis-full">
						<section class="flex min-h-dvh flex-col px-6 pb-8 pt-10">
							<div class="flex-1 flex flex-col justify-center">
								<div class="mx-auto w-full max-w-sm text-center">
									<div class="mb-8 aspect-4/3 rounded-2xl border bg-muted"></div>

									<h1 class="text-2xl font-semibold tracking-tight">{step.title}</h1>

									<p class="mt-3 text-sm text-muted-foreground">{step.body}</p>
								</div>
							</div>
						</section>
					</Carousel.Item>
				{/each}
			</Carousel.Content>
		</Carousel.Root>
	</div>

	<div class="border-t bg-background px-6 py-4">
		<div class="mx-auto flex max-w-sm items-center justify-center gap-2 pb-4">
			{#each steps as _, i}
				<!-- svelte-ignore element_invalid_self_closing_tag -->
				<button
					type="button"
					class={`h-2 rounded-full transition-all ${
            i === current ? "w-6 bg-foreground" : "w-2 bg-muted-foreground/30"
          }`}
					aria-label={`Go to step ${i + 1}`}
					aria-current={i === current ? "true" : undefined}
					onclick={() => goTo(i)}
				/>
			{/each}
		</div>

		<div class="mx-auto flex max-w-sm items-center justify-between gap-3">
			<Button variant="ghost" onclick={isFirst ? undefined : prev} disabled={isFirst}>
				Back
			</Button>

			{#if !isLast}
				<Button onclick={next}> Next </Button>
			{:else}
				<Button onclick={finish}> Get started </Button>
			{/if}
		</div>
	</div>
</div>
