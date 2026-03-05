<script lang="ts">
	const witnessLine = new Intl.DateTimeFormat('en-US', {
		weekday: 'long',
		month: 'long',
		day: 'numeric',
	}).format(new Date())
</script>

<div class="space-y-8 text-center">
	<p class="text-[0.8rem] tracking-[0.16em] text-foreground/48">{witnessLine}</p>

	<div class="space-y-4">
		<p class="font-serif text-[2.8rem] tracking-[0.22em] text-foreground">MEMENTO MORI</p>
		<p class="mx-auto max-w-xs text-[1.02rem] leading-7 text-foreground/68">
			This day is finite. Let it be lived.
		</p>
	</div>

	<div class="mx-auto h-px w-16 bg-[rgba(63,53,43,0.18)]"></div>

	<div class="mx-auto max-w-xs space-y-3">
		<p class="text-[0.92rem] leading-7 text-foreground/72">One thing worth doing with this day:</p>
		<p class="font-serif text-[1.5rem] leading-8 text-foreground/86">
			Begin by making the surface in front of you honest and alive.
		</p>
	</div>
</div>
