<script lang="ts">
  import { onMount } from 'svelte'
  import { goto } from '$app/navigation'
  import { startAuthStore, authState } from '$lib/features/auth'
  import FeedbackBox from '$lib/components/onboarding/FeedbackBox.svelte'
  import StepCard from '$lib/components/onboarding/StepCard.svelte'
  import type { CarouselAPI } from '$lib/components/ui/carousel/context.js'
  import {
    connectDevice,
    connState,
    disconnectDevice,
    syncConnectionState,
  } from '$lib/features/connection'
  import {
    type LocationStatus,
    prefsState,
    refreshLocationState,
    requestLocationState,
    startPrefsStore,
  } from '$lib/features/prefs'
  import { toCmdError, type DeviceMode } from '$lib/tauri'
  import {
    Alert,
    AlertDescription,
    AlertTitle,
  } from '$lib/components/ui/alert/index.js'
  import { Button } from '$lib/components/ui/button/index.js'
  import * as Carousel from '$lib/components/ui/carousel/index.js'
  import { Input } from '$lib/components/ui/input/index.js'
  import * as NativeSelect from '$lib/components/ui/native-select/index.js'

  type StepId = 'welcome' | 'name' | 'device' | 'location' | 'integrations'
  type PendingAction = 'connect' | 'disconnect' | 'location'
  type Step = {
    id: StepId
    title: string
    description: string
    nextLabel: string
  }
  type IntegrationProvider = 'github' | 'twitch' | 'google'
  type IntegrationCard = {
    id: IntegrationProvider
    label: string
    description: string
    connected: boolean
  }

  const locationStatusLabels: Record<LocationStatus, string> = {
    granted: 'Granted',
    denied: 'Denied',
    prompt: 'Not enabled yet',
    'prompt-with-rationale': 'Not enabled yet',
    'not-available': 'Not available on this platform',
  }

  const deviceModes: Array<{ value: DeviceMode; label: string }> = [
    { value: 'RealDevice', label: 'Real device' },
    { value: 'Simulator', label: 'Simulator' },
  ]

  const steps: Step[] = [
    {
      id: 'welcome',
      title: 'Set up Memori',
      description: 'A quick setup before you head to the device screen.',
      nextLabel: 'Start',
    },
    {
      id: 'name',
      title: 'Your name',
      description: 'This is what Memori uses when it greets you.',
      nextLabel: 'Continue',
    },
    {
      id: 'device',
      title: 'Connect device',
      description:
        'Use the same device connection API as the main device screen.',
      nextLabel: 'Continue',
    },
    {
      id: 'location',
      title: 'Location',
      description:
        'Optional, but useful for local weather and commute context.',
      nextLabel: 'Continue',
    },
    {
      id: 'integrations',
      title: 'Integrations',
      description:
        'These stay optional for now. You can finish once the device is connected.',
      nextLabel: 'Finish',
    },
  ]

  const integrationDetails: Array<
    Pick<IntegrationCard, 'id' | 'label' | 'description'>
  > = [
    {
      id: 'github',
      label: 'GitHub',
      description: 'Planned for repo and notification syncing.',
    },
    {
      id: 'twitch',
      label: 'Twitch',
      description: 'Planned for stream and account status.',
    },
    {
      id: 'google',
      label: 'Google',
      description: 'Planned for calendar and related context.',
    },
  ]

  let isReady = $state(false)
  let api = $state<CarouselAPI | null>(null)
  let current = $state(0)
  let pendingAction = $state<PendingAction | null>(null)
  let deviceMode = $state<DeviceMode>('RealDevice')
  let initializationMessage = $state('')
  let deviceMessage = $state('')
  let locationMessage = $state('')
  let deviceTone = $state<'neutral' | 'success' | 'error'>('neutral')
  let locationTone = $state<'neutral' | 'success' | 'error'>('neutral')

  const currentStep = $derived(steps[current] ?? steps[0])
  const isFirst = $derived(current === 0)
  const isLast = $derived(current === steps.length - 1)
  const isBusy = $derived(pendingAction !== null)
  const canAdvance = $derived.by(() => {
    switch (currentStep.id) {
      case 'name':
        return prefsState.name.trim().length > 0
      case 'device':
      case 'integrations':
        return connState.isConnected
      default:
        return true
    }
  })
  const integrationCards = $derived<IntegrationCard[]>(
    integrationDetails.map((card) => ({
      ...card,
      connected: !!authState.usersByProvider[card.id],
    }))
  )
  const locationStatusLabel = $derived(
    locationStatusLabels[prefsState.locationStatus]
  )
  const locationSummary = $derived.by(() => {
    if (locationMessage) return locationMessage
    if (prefsState.lastKnownLocation) {
      const { latitude, longitude } = prefsState.lastKnownLocation.coords
      return `Last saved location: ${latitude.toFixed(5)}, ${longitude.toFixed(5)}`
    }
    return 'No saved location yet.'
  })
  const deviceStatusLabel = $derived(
    connState.isConnected ? 'Connected' : 'Not connected'
  )
  const deviceSummary = $derived(
    deviceMessage ||
      initializationMessage ||
      'Connect before finishing onboarding.'
  )

  onMount(() => {
    void initializePage()
  })

  function handleApi(nextApi: CarouselAPI | undefined) {
    if (!nextApi) return

    api = nextApi
    current = nextApi.selectedScrollSnap()

    nextApi.on('select', () => {
      current = nextApi.selectedScrollSnap()
    })
  }

  function appendInitializationMessage(message: string) {
    initializationMessage = initializationMessage
      ? `${initializationMessage} ${message}`
      : message
  }

  async function initializePage() {
    const [prefsStart, authStart] = await Promise.allSettled([
      startPrefsStore(),
      startAuthStore(),
    ])

    if (prefsStart.status === 'rejected') {
      appendInitializationMessage(
        `Preferences storage unavailable: ${toCmdError(prefsStart.reason)}`
      )
    }

    if (authStart.status === 'rejected') {
      appendInitializationMessage(
        `Account placeholders unavailable: ${toCmdError(authStart.reason)}`
      )
    }

    if (prefsState.onboarded) {
      await goto('/device', { replaceState: true })
      return
    }

    await Promise.allSettled([
      syncConnectionState().match(
        () => undefined,
        (error) => {
          appendInitializationMessage(`Connection status unavailable: ${error}`)
        }
      ),
      refreshLocationState().catch((error) => {
        appendInitializationMessage(
          `Location status unavailable: ${toCmdError(error)}`
        )
      }),
    ])

    isReady = true
  }

  function prev() {
    if (!api || isFirst) return
    api.scrollPrev()
  }

  function next() {
    if (!api || !canAdvance || isLast) return
    api.scrollNext()
  }

  function goTo(index: number) {
    if (!api) return
    if (index <= current) {
      api.scrollTo(index)
      return
    }

    if (index === current + 1 && canAdvance) {
      api.scrollTo(index)
    }
  }

  function advance() {
    if (isLast) {
      void finishOnboarding()
      return
    }

    next()
  }

  async function finishOnboarding() {
    if (!connState.isConnected) return
    prefsState.onboarded = true
    await goto('/device', { replaceState: true })
  }

  async function runPendingAction(
    action: PendingAction,
    task: () => Promise<void>
  ): Promise<void> {
    pendingAction = action
    try {
      await task()
    } finally {
      pendingAction = null
    }
  }

  async function handleConnect() {
    deviceMessage = ''
    deviceTone = 'neutral'

    await runPendingAction('connect', async () => {
      await connectDevice(deviceMode).match(
        () => {
          deviceTone = 'success'
          deviceMessage = 'Device connected.'
        },
        (error) => {
          deviceTone = 'error'
          deviceMessage = `Connection failed: ${error}`
        }
      )
    })
  }

  async function handleDisconnect() {
    await runPendingAction('disconnect', async () => {
      await disconnectDevice().match(
        () => {
          deviceTone = 'neutral'
          deviceMessage = 'Device disconnected.'
        },
        (error) => {
          deviceTone = 'error'
          deviceMessage = `Disconnect failed: ${error}`
        }
      )
    })
  }

  async function handleLocationRequest() {
    locationMessage = ''
    locationTone = 'neutral'

    await runPendingAction('location', async () => {
      try {
        const position = await requestLocationState()

        if (!position) {
          locationTone = 'neutral'
          locationMessage =
            prefsState.locationStatus === 'denied'
              ? 'Location is denied. You can continue and enable it later.'
              : 'Location was not granted.'
          return
        }

        locationTone = 'success'
        locationMessage = `Saved ${position.coords.latitude.toFixed(5)}, ${position.coords.longitude.toFixed(5)}`
      } catch (error) {
        locationTone = 'error'
        locationMessage = `Location update failed: ${toCmdError(error)}`
      }
    })
  }
</script>

<svelte:head>
  <title>Set Up Memori</title>
</svelte:head>

{#if !isReady}
  <section
    class="flex min-h-dvh items-center justify-center bg-background px-6"
  >
    <div class="w-full max-w-sm space-y-3 text-center">
      <p class="text-xl font-semibold tracking-tight">Memori</p>
      <p class="text-sm text-muted-foreground">Preparing onboarding…</p>
      {#if initializationMessage}
        <p class="text-sm text-muted-foreground">{initializationMessage}</p>
      {/if}
    </div>
  </section>
{:else}
  <div class="flex min-h-dvh flex-col bg-background">
    <div class="flex-1 px-4 pb-4 pt-6">
      <div class="mx-auto flex h-full w-full max-w-sm flex-col">
        <div class="mb-4 space-y-1">
          <p class="text-xs text-muted-foreground">
            Step {current + 1} of {steps.length}
          </p>
          <h1 class="text-2xl font-semibold tracking-tight">
            {currentStep.title}
          </h1>
          <p class="text-sm text-muted-foreground">{currentStep.description}</p>
        </div>

        <Carousel.Root
          class="flex-1"
          opts={{
            align: 'start',
            loop: false,
            dragFree: false,
            watchDrag: false,
          }}
          setApi={handleApi}
        >
          <Carousel.Content class="ml-0 h-full">
            <Carousel.Item class="basis-full pl-0">
              <StepCard
                title="Welcome"
                description="Keep setup minimal, then continue into the device screen."
              >
                <p class="text-sm text-muted-foreground">
                  This flow only covers the basics: your name, device
                  connection, location, and placeholder integrations.
                </p>

                {#if initializationMessage}
                  <Alert>
                    <AlertTitle>Startup note</AlertTitle>
                    <AlertDescription>{initializationMessage}</AlertDescription>
                  </Alert>
                {/if}
              </StepCard>
            </Carousel.Item>

            <Carousel.Item class="basis-full pl-0">
              <StepCard
                title="Name"
                description="Required before you continue."
              >
                <Input
                  bind:value={prefsState.name}
                  placeholder="Enter your name"
                  autocomplete="name"
                />
                <p class="text-sm text-muted-foreground">
                  This writes directly to preferences as you type.
                </p>
              </StepCard>
            </Carousel.Item>

            <Carousel.Item class="basis-full pl-0">
              <StepCard
                title="Device connection"
                description="Use the same connection flow as the device page."
                contentClass="space-y-4"
                rootClass=""
              >
                <div class="space-y-2">
                  <p class="text-sm font-medium">Connection mode</p>
                  <NativeSelect.Root bind:value={deviceMode} class="w-full">
                    {#each deviceModes as mode (mode.value)}
                      <NativeSelect.Option value={mode.value}
                        >{mode.label}</NativeSelect.Option
                      >
                    {/each}
                  </NativeSelect.Root>
                </div>

                <div class="space-y-2">
                  <p class="text-sm font-medium">Pairing code</p>
                  <Input />
                  <p class="text-sm text-muted-foreground">
                    Code pairing is not wired yet.
                  </p>
                </div>

                <div class="flex flex-wrap gap-2">
                  <Button
                    onclick={handleConnect}
                    disabled={isBusy || connState.isConnected}
                  >
                    {pendingAction === 'connect' ? 'Connecting…' : 'Connect'}
                  </Button>
                  <Button
                    variant="outline"
                    onclick={handleDisconnect}
                    disabled={isBusy || !connState.isConnected}
                  >
                    {pendingAction === 'disconnect'
                      ? 'Disconnecting…'
                      : 'Disconnect'}
                  </Button>
                </div>

                <FeedbackBox
                  title={deviceStatusLabel}
                  message={deviceSummary}
                  tone={deviceTone}
                />
              </StepCard>
            </Carousel.Item>

            <Carousel.Item class="basis-full pl-0">
              <StepCard
                title="Location"
                description="Optional. You can continue even if this stays unavailable."
                contentClass="space-y-4"
                rootClass=""
              >
                <FeedbackBox
                  title={locationStatusLabel}
                  message={locationSummary}
                  tone={locationTone}
                />

                <Button
                  variant="outline"
                  onclick={handleLocationRequest}
                  disabled={isBusy ||
                    prefsState.locationStatus === 'not-available'}
                >
                  {pendingAction === 'location'
                    ? 'Checking…'
                    : 'Enable location'}
                </Button>
              </StepCard>
            </Carousel.Item>

            <Carousel.Item class="basis-full pl-0">
              <StepCard
                title="Integrations"
                description="Optional placeholders for what comes next."
              >
                {#each integrationCards as card (card.id)}
                  <div class="rounded-md border px-3 py-3">
                    <div class="flex items-start justify-between gap-3">
                      <div class="space-y-1">
                        <p class="text-sm font-medium">{card.label}</p>
                        <p class="text-sm text-muted-foreground">
                          {card.description}
                        </p>
                      </div>
                      <Button variant="outline" disabled>
                        {card.connected ? 'Connected' : 'Coming soon'}
                      </Button>
                    </div>
                  </div>
                {/each}
              </StepCard>
            </Carousel.Item>
          </Carousel.Content>
        </Carousel.Root>
      </div>
    </div>

    <div class="border-t bg-background px-4 py-4">
      <div class="mx-auto flex max-w-sm items-center justify-center gap-2 pb-4">
        {#each steps as step, index (step.id)}
          <button
            type="button"
            class={`h-2 rounded-full ${
              index === current
                ? 'w-6 bg-foreground'
                : 'w-2 bg-muted-foreground/30'
            }`}
            aria-label={`Go to step ${index + 1}`}
            aria-current={index === current ? 'true' : undefined}
            onclick={() => goTo(index)}
          ></button>
        {/each}
      </div>

      <div class="mx-auto flex max-w-sm items-center justify-between gap-3">
        <Button variant="ghost" onclick={prev} disabled={isFirst || isBusy}
          >Back</Button
        >
        <Button onclick={advance} disabled={!canAdvance || isBusy}>
          {currentStep.nextLabel}
        </Button>
      </div>
    </div>
  </div>
{/if}
