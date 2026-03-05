<script lang="ts">
	import { Input } from '$lib/components/ui/input'

	let {
		name = $bindable(''),
	} = $props<{
		name?: string
	}>()

	const trimmedName = $derived(name.trim())
</script>

<div class="space-y-8">
	<div class="space-y-3">
		<p class="text-[0.78rem] uppercase tracking-[0.24em] text-foreground/48">Inscription</p>
		<p class="max-w-xs text-[0.98rem] leading-7 text-foreground/70">
			Give the device a name it can return to. Keep it plain and sincere.
		</p>
	</div>

	<label class="block space-y-3" for="onboarding-name">
		<span class="text-[0.92rem] leading-7 text-foreground/74">How should Memori greet you?</span>
		<Input
			id="onboarding-name"
			bind:value={name}
			placeholder="Ada, Kenric, Luna..."
			class="h-14 rounded-none border-x-0 border-b border-t-0 border-[rgba(63,53,43,0.22)] bg-transparent px-0 text-lg shadow-none focus-visible:border-[rgba(122,95,54,0.45)] focus-visible:ring-0"
			autocapitalize="words"
			autocomplete="name"
		/>
	</label>

	<div class="space-y-2">
		<p class="text-[0.78rem] uppercase tracking-[0.24em] text-foreground/45">Preview</p>
		<p class="font-serif text-[1.9rem] leading-8 text-foreground/88">
			{trimmedName ? `Hello, ${trimmedName}.` : 'Hello there.'}
		</p>
	</div>
</div>
