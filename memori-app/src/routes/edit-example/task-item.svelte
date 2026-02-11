<script lang="ts" module>
  export type TaskItem = {
    id: string
    description: string
  }

  interface ItemProps {
    task: TaskItem
    index: number
    group?: string | number
    isOverlay?: boolean
    data?: { group: string | number }
  }
</script>

<!-- svelte-ignore state_referenced_locally -->
<script lang="ts">
  import { cn } from '@/utils'

  import { useSortable } from '@dnd-kit-svelte/svelte/sortable'

  let { task, index: idx, group, isOverlay = false, data }: ItemProps = $props()

  const { ref, handleRef, isDragging, isDropTarget } = useSortable({
    id: task.id,
    index: () => idx,
    type: 'todo-item',
    accept: 'todo-item',
    group: group,
    data: data,
  })
</script>

<div class="relative select-none" {@attach ref}>
  <!-- original element - becomes invisible during drag but maintains shape -->
  <div
    class={cn('p-3 bg-white rounded-(--radius) flex justify-between', {
      invisible: isDragging.current && !isOverlay,
      'bg-orange-600/5!': isDropTarget.current,
    })}
  >
    <div class="">
      <p class="font-bold">
        {task.id}
      </p>
      {task.description}
    </div>
    <div class="text-gray-500 cursor-pointer" {@attach handleRef}>grip</div>
  </div>

  <!-- drag placeholder, shows under when overlay is dragged -->
  {#if !isOverlay && isDragging.current}
    <div class="flex items-center justify-center absolute inset-0">
      <!-- put anything here for dragging state -->
      <div class="bg-orange-600/10 rounded-(--radius)">Moving: {task.id}</div>
    </div>
  {/if}
</div>
