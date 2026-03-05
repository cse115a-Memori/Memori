<script lang="ts">
  import type { Snippet } from 'svelte'
  import * as Card from '$lib/components/ui/card/index.js'

  let {
    children,
  }: {
    children: Snippet
  } = $props()
</script>

<div class={`flex content-center justify-center`}>
  {@render children()}
</div>
