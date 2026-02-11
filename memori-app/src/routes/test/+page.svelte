<script lang="ts">
  import { DragDropProvider, DragOverlay } from '@dnd-kit-svelte/svelte'
  import { move } from '@dnd-kit/helpers'
  import { sensors } from '$lib/sensors'
  import TaskItem from './task-item.svelte'

  let items = $state({
    listWidgets: [
      {
        id: 'erm',
        description: 'erm hi',
      },
    ],
    frameWidgets: [
      {
        id: 'im special',
        description: 'on it',
      },
    ],
  })
</script>

<DragDropProvider
  {sensors}
  onDragOver={(event) => {
    const { source } = event.operation
    if (source?.type === 'column') return
    items = move(items, event)
  }}
>
  <div class="grid gap-3 md:grid-cols-2">
    {#each Object.entries(items) as [columnId, nesteds], colIdx (columnId)}
      <h1>{columnId}</h1>
      {#each nesteds as nested, nestedIdx (nested.id)}
        <TaskItem task={nested} idx={nestedIdx} group={columnId} />
      {/each}
    {/each}
  </div>
</DragDropProvider>
