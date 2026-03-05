<script lang="ts">
	type FeedbackTone = 'neutral' | 'success' | 'error'

	const toneClasses: Record<FeedbackTone, string> = {
		neutral: 'border-border bg-muted/40 text-muted-foreground',
		success: 'border-emerald-200 bg-emerald-50 text-emerald-900',
		error: 'border-destructive/30 bg-destructive/5 text-destructive',
	}

	let {
		title,
		message,
		tone = 'neutral',
	}: {
		title: string
		message: string
		tone?: FeedbackTone
	} = $props()
</script>

<div class={`rounded-md border px-3 py-2 text-sm ${toneClasses[tone]}`}>
	<p class="font-medium">{title}</p>
	<p class="mt-1">{message}</p>
</div>
