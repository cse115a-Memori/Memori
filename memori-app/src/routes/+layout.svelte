<script lang="ts">
  import type { Component } from 'svelte'
  import { onMount } from 'svelte'
  import { Button } from '@/components/ui/button'
  import { startAuthStore } from '@/features/auth/store'
  import {
    connectDevice,
    connState,
    syncConnectionState,
  } from '@/features/connection'
  import { startGitHubStore } from '@/features/github'
  import { prefsState, startPrefsStore } from '@/features/prefs/store'
  import { startWidgetsStore } from '@/features/widgets/widgets-store'
  import { afterNavigate, goto, onNavigate } from '$app/navigation'
  // import { refreshLocationState } from '@/features/prefs/service'
  import { page } from '$app/state'

  import '../app.css'
  import { House, LoaderCircle, Settings, SquarePen } from '@lucide/svelte'

  const { children } = $props()
  type PrimaryNavItem = {
    href: string
    label: string
    icon: Component
    aliases?: string[]
  }

  const primaryNav: PrimaryNavItem[] = [
    { href: '/', label: 'Home', icon: House },
    { href: '/editor', label: 'Editor', icon: SquarePen, aliases: ['/device'] },
    {
      href: '/settings',
      label: 'Settings',
      icon: Settings,
      aliases: ['/testing'],
    },
  ]

  const pathname = $derived(page.url.pathname)
  const isOnboardingRoute = $derived(pathname.startsWith('/onboarding'))
  let isReady = $state(false)
  let isPrefsReady = $state(false)

  function navigateTo(href: string) {
    void goto(href)
  }

  function ensureOnboarding() {
    if (!isPrefsReady || prefsState.onboarded || page.url.pathname.startsWith('/onboarding')) {
      return
    }

    void goto('/onboarding', { replaceState: true })
  }

  function isNavActive({ href, aliases = [] }: PrimaryNavItem) {
    return [href, ...aliases].includes(pathname)
  }

  onNavigate((navigation) => {
    if (!document.startViewTransition) return

    return new Promise((resolve) => {
      document.startViewTransition(async () => {
        resolve()
        await navigation.complete
      })
    })
  })

  afterNavigate(() => {
    ensureOnboarding()
  })

  onMount(() => {
    void (async () => {
      try {
        await Promise.all([
          startPrefsStore().then(() => {
            isPrefsReady = true
            ensureOnboarding()
          }),
          startWidgetsStore(),
          startAuthStore(),
          startGitHubStore(),
          syncConnectionState(),
          // refreshLocationState(),
        ])

        if (connState.deviceCode !== '') {
          await connectDevice('RealDevice', connState.deviceCode).match(
            () => {
              connState.isConnected = true
            },
            () => {
              connState.isConnected = false
            }
          )
        }
      } catch (error) {
        console.error('Failed to start stores:', error)
      } finally {
        isReady = true
      }
    })()
  })

</script>

{#if isReady}
  <div class="h-dvh">
    {#if isOnboardingRoute}
      {@render children?.()}
    {:else}
      <div class="mx-auto w-full max-w-screen-sm px-4 pt-6 pb-24 md:pb-6">
        <div class="mb-4 flex flex-wrap items-center gap-2">
          {#each primaryNav as item (item.href)}
            {@render navLinks(item)}
          {/each}
        </div>
        {@render children?.()}
      </div>
      <nav
        class="fixed inset-x-0 bottom-0 z-40 border-t bg-background/95 px-4 pt-3 pb-[calc(env(safe-area-inset-bottom)+0.75rem)] backdrop-blur supports-[backdrop-filter]:bg-background/80 md:hidden"
        aria-label="Primary"
      >
        <div class="mx-auto grid w-full max-w-screen-sm grid-cols-3 gap-2">
          {#each primaryNav as item (item.href)}
            {@render mobileNavLink(item)}
          {/each}
        </div>
      </nav>
    {/if}
  </div>
{:else}
  <div class="flex min-h-dvh items-center justify-center">
    <LoaderCircle class="animate-spin" />
  </div>
{/if}

{#snippet navLinks(item: PrimaryNavItem)}
  <Button
    variant={isNavActive(item) ? 'secondary' : 'ghost'}
    onclick={() => navigateTo(item.href)}
    class="hidden gap-2 md:inline-flex"
  >
    <item.icon class="size-4" />
    {item.label}
  </Button>
{/snippet}

{#snippet mobileNavLink(item: PrimaryNavItem)}
  <Button
    variant={isNavActive(item) ? 'secondary' : 'ghost'}
    onclick={() => navigateTo(item.href)}
    class="h-14 flex-col gap-1 rounded-2xl px-2 text-[0.7rem]"
    aria-current={isNavActive(item) ? 'page' : undefined}
  >
    <item.icon class="size-4" />
    <span>{item.label}</span>
  </Button>
{/snippet}
