<script lang="ts" module>
  interface ItemProps {
    widget: Memori.Widget | undefined
    index: number
    group?: string | number
    isOverlay?: boolean
    data?: { group: string | number }
  }
</script>

<!-- svelte-ignore state_referenced_locally -->
<script lang="ts">
  import { cn } from '@/utils'

  import { GripVertical } from '@lucide/svelte'
  import { useSortable } from '@dnd-kit-svelte/svelte/sortable'
  import { Button } from '@/components/ui/button'

  let {
    widget,
    index: idx,
    group,
    isOverlay = false,
    data,
  }: ItemProps = $props()

  const { ref, handleRef, isDragging, isDropTarget } = useSortable({
    id: (widget && widget.id) || 'hi',
    index: () => idx,
    type: 'widget-layout-slot',
    accept: ['widget-item', 'widget-layout-slot'],
    group: group,
    data: data,
  })

  const selectWidgetItem = () => {
    console.log('selected')
  }
</script>

<div class="relative select-none" {@attach ref}>
  {#if widget}
    <!-- original element - becomes invisible during drag but maintains shape -->
    <div
      class={cn('p-3 bg-white rounded-(--radius) flex justify-between', {
        invisible: isDragging.current && !isOverlay,
        'bg-orange-600/5!': isDropTarget.current,
      })}
    >
      <div class="">
        {#if 'Name' in widget.kind}
          <div class="outline-2 bg-orange-200 p-5">
            Name: {widget.kind.Name.name}
          </div>
        {:else if 'Clock' in widget.kind}
          <div class="outline-2 bg-orange-200 p-5">
            Clock:
            {widget.kind.Clock.hours} hr
            {widget.kind.Clock.minutes} min
            {widget.kind.Clock.seconds} sec
          </div>
        {/if}
      </div>
      <div class="text-gray-500 cursor-pointer" {@attach handleRef}>
        <GripVertical />
      </div>
    </div>

    <!-- drag placeholder, shows under when overlay is dragged -->
    {#if !isOverlay && isDragging.current}
      <div class="flex items-center justify-center absolute inset-0">
        <!-- put anything here for dragging state -->
        <div class="bg-orange-600/10 rounded-(--radius)">
          Moving: {widget.id}
        </div>
      </div>
    {/if}
  {:else}
    <Button>Select</Button>
  {/if}
</div>
