<script lang="ts">
  import { onMount } from 'svelte'
  import { Button } from '@/components/ui/button'
  import { startAppStore } from '@/stores/app-store'
  import { startAuthStore } from '@/stores/auth-store'
  import { onNavigate } from '$app/navigation'
  import { page } from '$app/state'
  import '../app.css'

  const { children } = $props()

  onMount(() => {
    void Promise.all([startAppStore(), startAuthStore()]).catch((error) => {
      console.error('Failed to start stores:', error)
    })
  })

  onNavigate((navigation) => {
    if (!document.startViewTransition) return

    return new Promise((resolve) => {
      document.startViewTransition(async () => {
        resolve()
        await navigation.complete
      })
    })
  })
</script>

<div class="min-h-dvh">
  <div class="mx-auto w-full max-w-screen-sm px-4 py-6">
    {@render navLinks('/', 'Home')}
    {@render navLinks('/login', 'Login')}
    {@render navLinks('/device', 'Device')}
    {@render navLinks('/widgets', 'widgets')}
    {@render navLinks('/location', 'Location')}

    {@render children?.()}
  </div>
</div>

{#snippet navLinks(route: string, name: string)}
  <Button
    variant="link"
    href={route}
    class={`${page.url.pathname == route ? 'font-bold' : ''} transition-all`}
    >{name}</Button
  >
{/snippet}
