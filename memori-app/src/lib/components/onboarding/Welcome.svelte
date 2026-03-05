<script lang="ts">
	import Button from '../ui/button/button.svelte'

	let {
		prev,
		next,
	}: {
		prev: () => void
		next: () => void
	} = $props()
</script>

<div class="flex flex-col">
	<section>
		<h1>Memori</h1>
		<p class="text-slate-600">
			<span class="italic ">Memento Mori</span>
			&ndash; Remember you must die.
		</p>
	</section>

	<Button onclick={() => next()}>Get Started</Button>
</div>
