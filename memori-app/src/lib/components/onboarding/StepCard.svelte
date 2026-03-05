<script lang="ts">
  import type { Snippet } from 'svelte'
  import * as Card from '$lib/components/ui/card/index.js'

  let {
    title,
    description,
    rootClass = 'justify-center',
    contentClass = 'space-y-3',
    children,
  }: {
    title: string
    description: string
    rootClass?: string
    contentClass?: string
    children: Snippet
  } = $props()
</script>

<div class="flex h-full flex-col">
  <Card.Root class={`flex flex-1 flex-col ${rootClass}`}>
    <Card.Header class="space-y-1">
      <Card.Title>{title}</Card.Title>
      <Card.Description>{description}</Card.Description>
    </Card.Header>
    <Card.Content class={contentClass}>
      {@render children()}
    </Card.Content>
  </Card.Root>
</div>
