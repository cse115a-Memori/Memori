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

{@render navLinks('/', 'Home')}
{@render navLinks('/ble', 'ble')}
{@render navLinks('/test', 'test')}

{#snippet navLinks(route: string, name: string)}
	<Button
		variant="link"
		href={route}
		class={`${page.url.pathname == route ? 'font-bold' : ''} transition-all`}
	>{name}</Button>
{/snippet}

{@render children?.()}
