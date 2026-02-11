<script lang="ts" module>
  import { CollisionPriority } from '@dnd-kit/abstract'
  import { useSortable } from '@dnd-kit-svelte/svelte/sortable'
  import type { Snippet } from 'svelte'
  import { cn } from '@/utils'

  interface ColumnItemProps {
    children: Snippet<[isDragging: boolean]>
    data: { title: string; description: string }
    class?: string
    index: number
    id: string | number
    isOverlay?: boolean
  }
</script>

<!-- svelte-ignore state_referenced_locally -->
<script lang="ts">
  let {
    id,
    class: className,
    data,
    children,
    index: idx,
    isOverlay = false,
  }: ColumnItemProps = $props()

  const { ref, handleRef, isDragging } = useSortable({
    id: id,
    index: () => idx,
    type: 'column',
    accept: ['todo-item', 'column'],
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
          <span class="size-2.5 bg-orange-600 rounded-full absolute -left-5"
          ></span>
          {data.title}
        </p>
        {data.description}
      </div>

      <div class="text-gray-500 cursor-pointer" {@attach handleRef}>grip</div>
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
