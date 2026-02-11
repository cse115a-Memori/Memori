<script lang="ts" module>
  import { CollisionPriority } from '@dnd-kit/abstract'
  import { useSortable } from '@dnd-kit-svelte/svelte/sortable'
  import type { Snippet } from 'svelte'
  import { cn } from '@/utils'

  interface ColumnItemProps {
    id: string | number
    children: Snippet<[isDragging: boolean]>
    data: { title: string; description: string }
    class?: string
    index: number
    isOverlay?: boolean
  }
</script>

<!-- svelte-ignore state_referenced_locally -->
<script lang="ts">
  let {
    id = 1,
    class: className,
    data,
    children,
    index: idx,
    isOverlay = false,
  }: ColumnItemProps = $props()

  const { ref, handleRef, isDragging } = useSortable({
    id: id,
    index: () => idx,
    type: 'widget-layout-frame',
    accept: ['widget-item', 'widget-widget-layout-slot', 'widget-layout-frame'],
    collisionPriority: CollisionPriority.Low,
  })
</script>

<div class="relative" {@attach ref}>
  <!-- original element, invisible during drag but maintains shape -->
  <div
    class={cn('p-5 pt-6 bg-[#F9F9F9] rounded-(--radius)', className, {
      invisible: isDragging.current && !isOverlay,
    })}
  >
    <div class="flex justify-between text-[#9E9E9E]">
      <div class="pl-5.5">
        <p class="text-lg font-bold relative flex items-start">
          {data.title}
        </p>
        {data.description}
      </div>
    </div>

    <div class="grid gap-2 mt-3">
      {@render children(isDragging.current)}
    </div>
  </div>

  {#if !isOverlay && isDragging.current}
    <div
      class="hidden md:block absolute inset-0 bg-orange-600/10 border border-dashed border-orange-600 rounded-(--radius)"
    ></div>
  {/if}
</div>
