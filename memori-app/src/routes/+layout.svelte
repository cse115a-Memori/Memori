<script lang="ts">
	import '../app.css'
  import Button from '$lib/components/ui/button/button.svelte'

  const { children } = $props()
</script>

{@render children()}
