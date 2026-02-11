<script lang="ts">
  import { type UseDroppableInput, useDroppable } from '@dnd-kit-svelte/svelte'
  import type { Snippet } from 'svelte'
  import type { ClassValue } from 'svelte/elements'

  interface DroppableProps extends UseDroppableInput {
    children: Snippet
    class?: ClassValue
  }

  let { children, class: className, ...rest }: DroppableProps = $props()

  const { ref } = useDroppable(rest)
</script>

<!-- <div class={className} {@attach ref}> -->
<div class={className} {@attach ref}>
  {@render children()}
</div>
