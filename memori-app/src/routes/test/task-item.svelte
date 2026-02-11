<script lang="ts" module>
  export type TaskItem = {
    id: string
    description: string
  }

  interface ItemProps {
    task: TaskItem
    idx: number
    group: string
  }
</script>

<script lang="ts">
  import { useSortable } from '@dnd-kit-svelte/svelte/sortable'

  let { task, idx, group }: ItemProps = $props()

  const { ref, handleRef, isDragging, isDropTarget } = useSortable({
    // svelte-ignore state_referenced_locally
    id: task.id,
    index: () => idx,
    type: 'todo-item',
    accept: 'todo-item',
    // svelte-ignore state_referenced_locally
    group: group,
  })
</script>

<div class="relative select-none" {@attach ref}></div>
