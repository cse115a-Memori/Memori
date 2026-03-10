<script lang="ts">
	import type { CarouselAPI } from '$lib/components/ui/carousel/context.js'
	import * as Carousel from '$lib/components/ui/carousel/index.js'
	import { Button } from '@/components/ui/button'
	import { Input } from '@/components/ui/input'
	import { stepFrameCls } from './step-frame-classes'
	import { prefsState } from '$lib/features/prefs'
	import { onMount } from 'svelte'
	import { goto } from '$app/navigation'
	import { LoaderCircle } from '@lucide/svelte'

	import * as InputOTP from '$lib/components/ui/input-otp/index.js'

	import { connState, connectDevice } from '@/features/connection'
	import { requestLocationState } from '@/features/prefs/service'
	import { playFailedSound, playSuccessSound } from '@/features/sound'
	import { commands, toCmdError, type DeviceMode, tryCmd, type UserInfo } from '@/tauri'
	import * as Field from '$lib/components/ui/field/index.js'
	import * as NativeSelect from '$lib/components/ui/native-select/index.js'
	import { authState, login } from '@/features/auth'

	let currStepIdx = $state(0)
	let isLoading = $state(false)
	let errMsg = $state('')
	let pairingCode = $state('')

	$inspect(pairingCode)

	// INTEGRATION

	$inspect(authState.usersByProvider['twitch'])
	let isLoggedInGithub = $derived(authState.usersByProvider['github'] !== undefined)
	let isLoggedInTwitch = $derived(authState.usersByProvider['twitch'] !== undefined)
	let isLocationEnabled = $derived(prefsState.locationStatus === 'granted')

	type PendingAction = 'Github' | 'Twitch' | 'Location'
	let pendingOp = $state<PendingAction | null>(null)

	async function enableLocation() {
		pendingOp = 'Location'
		errMsg = ''
		try {
			await requestLocationState()
		} catch (error) {
			errMsg = toCmdError(error)
		} finally {
			pendingOp = null
		}
	}

	async function loginTwitch() {
		errMsg = ''
		pendingOp = 'Twitch'
		await login('twitch').match(
			nextUser => {
				console.log('logged into twitch')
			},
			error => {
				errMsg = `Twitch login failed: ${error}`
			}
		)
		pendingOp = null
	}

	async function loginGithub() {
		errMsg = ''
		pendingOp = 'Github'
		await login('github').match(
			nextUser => {
				console.log('logged into github')
			},
			error => {
				errMsg = `Github login failed: ${error}`
			}
		)
		pendingOp = null
	}

	// Carousel API

	let api = $state<CarouselAPI | null>(null)

	function handleApi(nextApi: CarouselAPI | undefined) {
		if (!nextApi) return

		api = nextApi
		currStepIdx = nextApi.selectedScrollSnap()

		nextApi.on('select', () => {
			currStepIdx = nextApi.selectedScrollSnap()
		})
	}
	function prev() {
		api?.scrollPrev()
	}

	function next() {
		api?.scrollNext()
	}

	async function connect() {
		errMsg = ''
		isLoading = true
		await connectDevice('RealDevice', pairingCode).match(
			() => {
				// playSuccessSound()
				connState.isConnected = true
				next()
			},
			error => {
				// dev only
				connState.isConnected = true
				next()

				errMsg = 'Pairing failed. Try again.'
				// playFailedSound()
			}
		)
		isLoading = false
	}

	function finishOnboarding() {
		prefsState.onboarded = true
		goto('/device')
	}

	onMount(() => {
		//dev only
		prefsState.onboarded = false
	})
</script>

<div class="page">
	<div class="content text-foreground/90">
		<div class="layout-grid grid h-full grid-rows-[minmax(0,1fr)_auto] px-4 pt-4">
			<Carousel.Root
				class="h-full min-h-0 w-full overflow-hidden rounded-2xl **:data-[slot='carousel-content']:h-full **:data-[slot='carousel-content']:min-h-0 [&_[data-slot='carousel-content']_[data-embla-container]]:h-full [&_[data-slot='carousel-content']_[data-embla-container]]:min-h-0 **:data-embla-slide:h-full **:data-embla-slide:min-h-0"
				opts={{
          align: 'start',
          loop: false,
          dragFree: false,
          watchDrag: false,
        }}
				setApi={handleApi}
			>
				<Carousel.Content class="h-full min-h-0">
					<Carousel.Item class="h-full min-h-0">
						{@render slideFrame(step1Content, step1Actions)}
					</Carousel.Item>

					<Carousel.Item class="h-full min-h-0">
						{@render slideFrame(step2Content, step2Actions)}
					</Carousel.Item>

					<Carousel.Item class="h-full min-h-0">
						{@render slideFrame(step3Content, step3Actions)}
					</Carousel.Item>

					<Carousel.Item class="h-full min-h-0">
						{@render slideFrame(step4Content, step4Actions)}
					</Carousel.Item>
				</Carousel.Content>
			</Carousel.Root>

			<div
				class="mx-auto mt-3 flex w-full max-w-sm items-center justify-center gap-2"
				aria-live="polite"
				aria-label="Onboarding progress"
			>
				{@render buttonHint(0)}
				{@render buttonHint(1)}
				{@render buttonHint(2)}
				{@render buttonHint(3)}
			</div>
		</div>
	</div>
</div>

{#snippet buttonHint(idx: number)}
	<button
		type="button"
		aria-label={`Go to step ${idx}`}
		aria-current={currStepIdx === idx ? 'true' : undefined}
		class={`h-2.5 min-h-2.5 rounded-full transition-all ${
      currStepIdx === idx ? 'w-8 bg-foreground' : 'w-2 bg-muted-foreground/40'
    }`}
	></button>
{/snippet}

{#snippet slideFrame(content: any, actions: any)}
	<section class="relative h-full min-h-0">
		<div class={stepFrameCls.topRegion}>{@render content()}</div>

		<div class={stepFrameCls.bottomRegion}>
			<div class={stepFrameCls.actionContainer}>{@render actions()}</div>
		</div>
	</section>
{/snippet}

{#snippet step1Content()}
	<h1
		class="font-serif text-[2.35rem] leading-[1.08] tracking-[0.05em] text-foreground"
	>
		Memori
	</h1>
{/snippet}

{#snippet step1Actions()}
	<div class="grid w-full grid-cols-1 gap-3">
		<Button
			onclick={next}
			class="h-12 w-full bg-foreground text-background hover:bg-foreground/90"
			>Begin</Button
		>
	</div>
{/snippet}

{#snippet step2Content()}
	<div class="w-full space-y-4">
		<p class="text-xs font-medium uppercase tracking-[0.24em] text-foreground/50">
			Step 2
		</p>
		<h1
			class="font-serif text-[2.15rem] leading-[1.1] tracking-[0.03em] text-foreground/94"
		>
			Your Name
		</h1>
		<label for="user-name" class="sr-only">Your name</label>
		<Input
			id="user-name"
			bind:value={prefsState.name}
			type="text"
			autocomplete="name"
			placeholder="Your name"
			class="mx-auto max-w-xs text-center"
		/>
	</div>
{/snippet}

{#snippet step2Actions()}
	<div class="grid w-full grid-cols-2 gap-3">
		<Button
			variant="outline"
			class="h-12 w-full border-border/60 text-foreground/90"
			onclick={prev}
		>
			Back
		</Button>

		<Button
			onclick={next}
			class="h-12 w-full bg-foreground text-background hover:bg-foreground/90"
			>Next</Button
		>
	</div>
{/snippet}

{#snippet step3Content()}
	<div class="w-full space-y-4 pt-20">
		<p class="text-xs font-medium uppercase tracking-[0.24em] text-foreground/50">
			Step 3
		</p>
		<h1
			class="font-serif text-[2.15rem] leading-[1.1] tracking-[0.03em] text-foreground/94"
		>
			Connect Memori
		</h1>
		<div class="flex justify-center items-center flex-col gap-1">
			<InputOTP.Root maxlength={4} bind:value={pairingCode}>
				{#snippet children({ cells })}
					<InputOTP.Group>
						{#each cells.slice(0, 4) as cell (cell)}
							<InputOTP.Slot {cell} />
						{/each}
					</InputOTP.Group>
				{/snippet}
			</InputOTP.Root>
			<p class="text-foreground/50 text-sm">Enter device pairing code.</p>
		</div>
		<span class="flex flex-col gap-2 justify-center items-center">
			<Button onclick={connect} class="h-10 w-32" disabled={pairingCode.length != 4}>
				{#if isLoading}
					<LoaderCircle class="animate-spin" />
				{:else}
					Connect
				{/if}
			</Button>
			{#if errMsg}
				<p class="text-red-600/70">{errMsg}</p>
			{/if}
		</span>
	</div>
{/snippet}

{#snippet step3Actions()}
	<div class="grid w-full grid-cols-1 gap-3">
		<Button
			variant="outline"
			class="h-12 w-full border-border/60 text-foreground/90"
			onclick={prev}
		>
			Back
		</Button>
	</div>
{/snippet}

{#snippet step4Content()}
	<div class="w-full space-y-4 pt-20">
		<p class="text-xs font-medium uppercase tracking-[0.24em] text-foreground/50">
			Step 4
		</p>
		<h1
			class="font-serif text-[2.15rem] leading-[1.1] tracking-[0.03em] text-foreground/94"
		>
			Integrations
		</h1>
		<div class="flex flex-col gap-3 justify-center items-center">
			{@render integrationCard(
        'Github',
        'Connect',
        'Connected',
        loginGithub,
        isLoggedInGithub
      )}
			{@render integrationCard(
        'Twitch',
        'Connect',
        'Connected',
        loginTwitch,
        isLoggedInTwitch
      )}
			{@render integrationCard(
        'Location',
        'Enable',
        'Enabled',
        enableLocation,
        isLocationEnabled
      )}
			{#if errMsg}
				<p class="mt-4 text-center">{errMsg}</p>
			{/if}
		</div>
	</div>
{/snippet}

{#snippet integrationCard(
  label: string,
  btnLabel: string,
  successLabel: string,
  func: () => any,
  success: boolean
)}
	{@const isLoading = pendingOp === label}
	<div class="flex justify-between items-center gap-1 w-full max-w-52">
		{label}:
		<Button
			variant="outline"
			onclick={func}
			class="w-32"
			disabled={(!isLoading && pendingOp !== null) || success}
		>
			{#if isLoading}
				<LoaderCircle class="animate-spin" />
			{:else if success}
				{successLabel}
			{:else}
				{btnLabel}
			{/if}
		</Button>
	</div>
{/snippet}

{#snippet step4Actions()}
	<div class="grid w-full grid-cols-2 gap-3">
		<Button
			variant="outline"
			class="h-12 w-full border-border/60 text-foreground/90"
			onclick={prev}
		>
			Back
		</Button>

		<Button
			onclick={finishOnboarding}
			class="h-12 w-full bg-foreground text-background"
		>
			{#if isLoading}
				<LoaderCircle class="animate-spin" />
			{:else}
				Done
			{/if}
		</Button>
	</div>
{/snippet}

<style>
	.page {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 100dvh;
		padding: clamp(2.5rem, 6vw, 5rem);
		background: var(--background);
		color: var(--foreground);
		overflow-x: clip;
		overflow-y: clip;
	}

	.page::before {
		content: '';
		position: absolute;
		inset: -20%;
		background:
			radial-gradient(60% 50% at 50% 0%, oklch(0.99 0.01 85 / 0.55) 0%, transparent 70%),
			radial-gradient(
				40% 35% at 85% 10%,
				oklch(0.98 0.02 85 / 0.35) 0%,
				transparent 60%
			);
		pointer-events: none;
	}

	.page::after {
		content: '';
		position: absolute;
		inset: 0;
		background-image: repeating-linear-gradient(
			0deg,
			oklch(0.8 0.02 85 / 0.03) 0,
			oklch(0.8 0.02 85 / 0.03) 1px,
			transparent 1px,
			transparent 3px
		);
		opacity: 0.35;
		pointer-events: none;
	}

	.content {
		position: relative;
		z-index: 1;
		width: min(42rem, 100%);
		display: grid;
		gap: 2.5rem;
		min-height: min(46rem, calc(100dvh - (2 * clamp(2.5rem, 6vw, 5rem))));
		text-align: center;
	}

	.content::before {
		content: '';
		position: absolute;
		width: min(34rem, 75vw);
		aspect-ratio: 1;
		border: 1px solid oklch(0.7 0.04 85 / 0.22);
		border-radius: 999px;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -70%);
		pointer-events: none;
	}

	.layout-grid {
		height: 100%;
		min-height: 0;
	}

	@media (max-width: 640px) {
		.page {
			align-items: flex-start;
		}
	}
</style>
