<script lang="ts">
  import { Button } from '@/components/ui/button'
  import { onNavigate } from '$app/navigation'
  import { page } from '$app/state'
  import '../app.css'

  const { children } = $props()

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
    {@render navLinks('/test', 'Test')}
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
