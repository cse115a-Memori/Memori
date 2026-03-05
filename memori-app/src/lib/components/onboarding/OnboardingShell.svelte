<script lang="ts">
	import type { Snippet } from 'svelte'
	import { Button } from '$lib/components/ui/button'
	import { Progress } from '$lib/components/ui/progress'

	type Props = {
		stepNumber: number
		totalSteps: number
		eyebrow: string
		title: string
		description: string
		canGoBack: boolean
		canAdvance: boolean
		onBack: () => void
		onAdvance: () => void
		advanceLabel: string
		backLabel?: string
		footerNote?: string
		children?: Snippet
	}

	let {
		stepNumber,
		totalSteps,
		eyebrow,
		title,
		description,
		canGoBack,
		canAdvance,
		onBack,
		onAdvance,
		advanceLabel,
		backLabel = 'Back',
		footerNote,
		children,
	}: Props = $props()

	const progressValue = $derived((stepNumber / totalSteps) * 100)
</script>

<section class="onboarding-shell relative isolate min-h-dvh overflow-hidden">
	<div class="onboarding-shell__grain"></div>
	<div class="onboarding-shell__watermark" aria-hidden="true">
		<div class="onboarding-shell__watermark-line"></div>
		<div class="onboarding-shell__watermark-top"></div>
		<div class="onboarding-shell__watermark-bottom"></div>
	</div>

	<div
		class="relative mx-auto flex min-h-dvh w-full max-w-[28rem] flex-col px-6 pb-[calc(env(safe-area-inset-bottom)+1.25rem)] pt-[calc(env(safe-area-inset-top)+1.5rem)]"
	>
		<header class="space-y-6">
			<div class="flex items-center justify-between gap-4 text-[0.72rem] uppercase tracking-[0.24em] text-foreground/48">
				<p>{eyebrow}</p>
				<p>{stepNumber} / {totalSteps}</p>
			</div>

			<div class="space-y-4">
				<Progress value={progressValue} class="h-px bg-[rgba(63,53,43,0.1)]" />
				<div class="space-y-3">
					<h1 class="font-serif text-[2.35rem] leading-[1.08] tracking-[0.05em] text-balance text-foreground">
						{title}
					</h1>
					<p class="text-[0.97rem] leading-7 text-foreground/68">{description}</p>
				</div>
			</div>
		</header>

		<main class="flex flex-1 flex-col justify-center py-10">
			{@render children?.()}
		</main>

		<footer class="space-y-4">
			{#if footerNote}
				<p class="text-sm leading-6 text-foreground/56">{footerNote}</p>
			{/if}

			<div class="flex items-center gap-3 pt-2">
				<Button
					variant="ghost"
					class="h-12 flex-1 rounded-none border border-transparent bg-transparent px-0 text-foreground/62 shadow-none hover:bg-transparent hover:text-foreground"
					disabled={!canGoBack}
					onclick={onBack}
				>
					{backLabel}
				</Button>

				<Button
					class="h-12 flex-[1.4] rounded-none border border-[rgba(122,95,54,0.34)] bg-[rgba(122,95,54,0.08)] text-foreground shadow-none hover:bg-[rgba(122,95,54,0.14)]"
					disabled={!canAdvance}
					onclick={onAdvance}
				>
					{advanceLabel}
				</Button>
			</div>
		</footer>
	</div>
</section>

<style>
	.onboarding-shell {
		background:
			linear-gradient(180deg, rgba(251, 248, 241, 0.985) 0%, rgba(245, 238, 226, 0.99) 100%);
	}

	.onboarding-shell__grain {
		position: absolute;
		inset: 0;
		background-image:
			radial-gradient(rgba(70, 56, 41, 0.045) 0.6px, transparent 0.6px),
			radial-gradient(rgba(70, 56, 41, 0.02) 0.8px, transparent 0.8px);
		background-position: 0 0, 12px 12px;
		background-size: 24px 24px;
		opacity: 0.18;
		pointer-events: none;
	}

	.onboarding-shell__watermark {
		position: absolute;
		inset: 50% auto auto 50%;
		width: 13rem;
		height: 16rem;
		transform: translate(-50%, -46%);
		opacity: 0.045;
		pointer-events: none;
	}

	.onboarding-shell__watermark-line {
		position: absolute;
		inset: 0 auto 0 50%;
		width: 1px;
		background: rgba(70, 56, 41, 0.9);
		transform: translateX(-50%);
	}

	.onboarding-shell__watermark-top,
	.onboarding-shell__watermark-bottom {
		position: absolute;
		left: 50%;
		width: 6.5rem;
		height: 5rem;
		border-left: 1px solid rgba(70, 56, 41, 0.9);
		border-right: 1px solid rgba(70, 56, 41, 0.9);
		transform: translateX(-50%);
	}

	.onboarding-shell__watermark-top {
		top: 2.35rem;
		border-top: 1px solid rgba(70, 56, 41, 0.9);
		clip-path: polygon(0 0, 100% 0, 60% 100%, 40% 100%);
	}

	.onboarding-shell__watermark-bottom {
		bottom: 2.35rem;
		border-bottom: 1px solid rgba(70, 56, 41, 0.9);
		clip-path: polygon(40% 0, 60% 0, 100% 100%, 0 100%);
	}
</style>
