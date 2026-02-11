<script lang="ts">
  import { move } from '@dnd-kit/helpers'
  import { DragDropProvider, DragOverlay } from '@dnd-kit-svelte/svelte'
  import { sensors } from '$lib/sensors'
  import TaskColumn from './task-column.svelte'
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
      {
        id: 'im not special',
        description: 'not on it',
      },
    ],
  })

  const meta: Record<string, { title: string; description: string }> = {
    listWidgets: { title: 'listWidgets', description: 'widgets list' },
    frameWidgets: { title: 'frameWidgets', description: 'widgets frame' },
  }
</script>

<DragDropProvider
  {sensors}
  onDragOver={(event) => {
    const { source } = event.operation
    if (source?.type === 'column') return
    items = move(items, event)
  }}
>
  <!-- placeholder boxes -->
  <div class="grid gap-3 md:grid-cols-2">
    {#each Object.entries(items) as [columnId, nesteds], colIdx (columnId)}
      <TaskColumn id={columnId} data={meta[columnId]} index={colIdx}>
        {#each nesteds as nested, nestedIdx (nested.id)}
          <TaskItem
            task={nested}
            index={nestedIdx}
            group={columnId}
            data={{ group: columnId }}
          />
        {/each}
      </TaskColumn>
    {/each}
  </div>

  <p class="text-sm text-center text-[#9E9E9E] font-medium pt-3">
    Drag and drop to reorder
  </p>

  <!-- where the actual content is, the cards you drag -->
  <DragOverlay>
    {#snippet children(source)}
      <!-- if has group, is item -->
      {#if source.data.group}
        {@const task = items[source.data.group as keyof typeof items]?.find(
          (task) => task.id === source.id
        )!}
        <TaskItem {task} index={0} isOverlay />
      {:else}
        <!-- else is a column -->
        <TaskColumn id={source.id} data={meta[source.id]} index={0} isOverlay>
          {#each items[source.id as keyof typeof items] as item, itemIdx (item.id)}
            <TaskItem
              task={item}
              index={itemIdx}
              group={source.id}
              data={{ group: source.id }}
            />
          {/each}
        </TaskColumn>
      {/if}
    {/snippet}
  </DragOverlay>
</DragDropProvider>
