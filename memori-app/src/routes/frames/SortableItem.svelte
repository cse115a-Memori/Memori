<script lang="ts">
  import {
    useSortable,
    type UseSortableInput,
  } from '@dnd-kit-svelte/svelte/sortable'
  import type { Snippet } from 'svelte'
  import {
    sortableCardBaseClasses,
    sortableCardContentClasses,
    sortableCardInteractiveClasses,
    sortableCardPlaceholderClasses,
    sortableCardPlaceholderTextClasses,
    sortableCardTitleClasses,
  } from './sortable-item-classes.ts'

  interface WidgetItem {
    id: string
    name: string
    content: string
  }

  interface Props extends UseSortableInput {
    widget: WidgetItem
    children?: Snippet
    isOverlay?: boolean
  }

  let { widget, children, isOverlay = false, ...rest }: Props = $props()

  const { ref, isDragging } = useSortable({ ...rest, feedback: 'move' })
</script>

<div class="relative select-none" {@attach ref}>
  <!-- Original element - becomes invisible during drag but maintains dimensions -->
  <div
    class={[
      sortableCardBaseClasses,
      sortableCardInteractiveClasses,
      { 'cursor-grabbing': isDragging.current || isOverlay },
      { invisible: isDragging.current && !isOverlay },
    ]}
  >
    <div class={sortableCardTitleClasses}>
      {widget.name}
    </div>
    <p class={sortableCardContentClasses}>
      {widget.content}
    </p>
  </div>

  <!-- Drag placeholder - set to match original dimensions -->
  {#if !isOverlay && isDragging.current}
    <div class="absolute inset-0 flex items-center justify-center">
      <!-- You can put any content here for the dragging state -->
      <div
        class={sortableCardPlaceholderClasses}
      >
        <span class={sortableCardPlaceholderTextClasses}>Moving: {widget.name}</span>
      </div>
    </div>
  {/if}
</div>
