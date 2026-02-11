<script lang="ts">
  import Droppable from './Droppable.svelte'
  import SortableItem from './SortableItem.svelte'
  import { CollisionPriority } from '@dnd-kit/abstract'
  import { DragDropProvider, DragOverlay } from '@dnd-kit-svelte/svelte'
  import { move } from '@dnd-kit/helpers'
  import { sensors } from '$lib/sensors'
  import { RestrictToWindowEdges } from '@dnd-kit-svelte/svelte/modifiers'
  import {
    sortableCardBaseClasses,
    sortableCardContentClasses,
    sortableCardTitleClasses,
  } from './sortable-item-classes.ts'

  function formatTime12h(date = new Date()) {
    let h = date.getHours()
    const m = String(date.getMinutes()).padStart(2, '0')
    const ampm = h >= 12 ? 'pm' : 'am'
    h = h % 12
    if (h === 0) h = 12
    return `${h}:${m} ${ampm}`
  }

  interface WidgetItem {
    id: string
    name: string
    content: string
  }

  const items = {
    widgets: [
      { id: 'task-1', name: 'Weather', content: '22°C' },
      { id: 'task-2', name: 'Time', content: formatTime12h() },
      { id: 'task-3', name: 'Bus', content: 'Next 19 in ~22 min' },
      {
        id: 'task-4',
        name: 'GitHub stats',
        content: 'PRs to review: 2\nIssues: 5',
      },
    ],
    'frame-widgets': [],
  }

  type WidgetItems = Record<string, WidgetItem[]>
  let widgets = $state<WidgetItems>(items)
  let activeWidget = $state<WidgetItem | null>(null)
  let lastDragOverKey = $state<string | null>(null)

  function getGroupIdFromDragItem(
    item: { id?: unknown; data?: unknown } | null | undefined
  ) {
    const dataGroup =
      item &&
      typeof item.data === 'object' &&
      item.data !== null &&
      'group' in item.data &&
      typeof (item.data as { group?: unknown }).group === 'string'
        ? (item.data as { group: string }).group
        : undefined

    if (dataGroup) return dataGroup
    if (item && typeof item.id === 'string' && item.id in widgets)
      return item.id
    return undefined
  }

  function getDragOverKey(event: {
    operation?: { source?: unknown; target?: unknown }
  }) {
    const source = event.operation?.source as
      | {
          id?: unknown
          data?: unknown
          index?: unknown
          shape?: { center?: { y?: number } }
          manager?: {
            dragOperation?: {
              shape?: { current?: { center?: { y?: number } } }
              position?: { current?: { y?: number } }
            }
          }
        }
      | undefined
    const target = event.operation?.target as
      | {
          id?: unknown
          data?: unknown
          index?: unknown
          shape?: { center?: { y?: number } }
        }
      | undefined
    const sourceGroup = getGroupIdFromDragItem(source)
    const targetGroup = getGroupIdFromDragItem(target)

    if (!source || !target || !sourceGroup || !targetGroup) return null

    const sourceIndex = typeof source.index === 'number' ? source.index : -1
    const targetIndex = typeof target.index === 'number' ? target.index : -1

    const operationShape = source.manager?.dragOperation?.shape?.current
    const operationPosition = source.manager?.dragOperation?.position?.current
    const pointerY = operationShape?.center?.y ?? operationPosition?.y
    const targetY = target.shape?.center?.y
    const side =
      pointerY !== undefined && targetY !== undefined
        ? pointerY > targetY
          ? 'after'
          : 'before'
        : 'none'

    return `${String(source.id)}:${sourceGroup}:${sourceIndex}->${String(target.id)}:${targetGroup}:${targetIndex}:${side}`
  }
</script>

<DragDropProvider
  {sensors}
  modifiers={[RestrictToWindowEdges]}
  onDragStart={(event) => {
    const source = event.operation.source
    const sourceGroup = getGroupIdFromDragItem(source)
    if (!source || !sourceGroup) return

    activeWidget =
      widgets[sourceGroup]?.find((entry) => entry.id === String(source.id)) ??
      null
    lastDragOverKey = null
  }}
  onDragOver={(event) => {
    const nextKey = getDragOverKey(event)
    if (!nextKey) return
    if (nextKey === lastDragOverKey) return

    const nextWidgets = move(widgets, event)
    if (nextWidgets !== widgets) widgets = nextWidgets
    lastDragOverKey = nextKey
  }}
  onDragEnd={(event) => {
    const nextWidgets = move(widgets, event)
    if (nextWidgets !== widgets) widgets = nextWidgets
    activeWidget = null
    lastDragOverKey = null
  }}
>
  <div class="grid gap-4 md:grid-cols-[1fr_3fr]">
    {@render taskList('widgets', 'Widgets', widgets['widgets'])}
    {@render taskList('frame-widgets', 'Frame', widgets['frame-widgets'])}
  </div>

  <DragOverlay>
    {#snippet children(source)}
      {#if activeWidget}
        {@const width = source.shape?.boundingRectangle.width}
        {@const height = source.shape?.boundingRectangle.height}
        <div
          class="relative select-none"
          style={width && height ? `width:${width}px;height:${height}px` : undefined}
        >
          <div class={[sortableCardBaseClasses, 'shadow-lg ring-2 ring-sky-300/60']}>
            <div class={sortableCardTitleClasses}>
              {activeWidget.name}
            </div>
            <p class={sortableCardContentClasses}>
              {activeWidget.content}
            </p>
          </div>
        </div>
      {/if}
    {/snippet}
  </DragOverlay>
</DragDropProvider>

{#snippet taskList(id: string, title: string, tasks: WidgetItem[])}
  <section class="bg-#F9F9F9 rd-3xl p-3 pt-6">
    <p class="text-lg fw-bold pb-3">{title}</p>

    <Droppable
      class="min-h-24"
      {id}
      type="column"
      accept="item"
      collisionPriority={CollisionPriority.Low}
    >
      {#if id == 'frame-widgets'}
        <div class="grid grid-cols-2 gap-2">
          {#each tasks as task, index (task.id)}
            <SortableItem
              widget={task}
              id={task.id}
              index={() => index}
              group={id}
              data={{ group: id }}
              type="item"
            />
          {/each}
        </div>
      {:else}
        <div class="flex flex-col gap-2">
          {#each tasks as task, index (task.id)}
            <SortableItem
              widget={task}
              id={task.id}
              index={() => index}
              group={id}
              data={{ group: id }}
              type="item"
            ></SortableItem>
          {/each}
        </div>
      {/if}
    </Droppable>
  </section>
{/snippet}
