<script lang="ts">
  import { DragDropProvider, DragOverlay } from '@dnd-kit-svelte/svelte'
  import * as Drawer from '$lib/components/ui/drawer/index'
  import { move } from '@dnd-kit/helpers'
  import { sensors } from '$lib/sensors'
  import WidgetItem from './WidgetItem.svelte'
  import WidgetLayoutSlot from './WidgetLayoutSlot.svelte'
  import { onMount } from 'svelte'
  import { commands } from '@/tauri'
  import { Button } from '@/components/ui/button'
  import WidgetLayoutFrame from './WidgetLayoutFrame.svelte'
  import {
    LAYOUT_TEMPLATES,
    type LayoutSlot,
    type WidgetCtx,
    slotsFor,
  } from './widget-utils'
  import * as Select from '$lib/components/ui/select/index.js'

  let error = $state('')
  let loading = $state(false)

  let selectedLayout: Memori.Layout['variant'] = $state('Full')
  let layout = $derived(LAYOUT_TEMPLATES[selectedLayout])

  const layoutVariants = Object.keys(LAYOUT_TEMPLATES) as Array<
    keyof typeof LAYOUT_TEMPLATES
  >

  // identify slot to add drawer widget to
  let selectedSlotId = $state<LayoutSlot['id']>()

  const selectSlotId = (slot: LayoutSlot['id']) => {
    selectedSlotId = slot
  }

  let widgets = $state<WidgetCtx>({
    widgetPool: [],
    widgetFrame: [],
  })

  onMount(() => {
    const load = async () => {
      const result = await commands.getWidgetKinds()
      if (result.status === 'ok') {
        widgets.widgetPool = result.data
      } else {
        error = result.error
      }
    }
    load()
  })

  const meta: Record<string, { title: string; description: string }> = {
    widgetPool: { title: 'widgetPool', description: 'widgets list' },
    widgetFrame: { title: 'widgetFrame', description: 'widgets frame' },
  }

  $inspect(widgets)
</script>

<Select.Root type="single" name="favoriteFruit" bind:value={selectedLayout}>
  <Select.Trigger class="">
    {selectedLayout}
  </Select.Trigger>
  <Select.Content>
    <Select.Group>
      <Select.Label>Fruits</Select.Label>
      {#each layoutVariants as layoutVariant (layoutVariant)}
        <Select.Item value={layoutVariant} label={layoutVariant}>
          {layoutVariant}
        </Select.Item>
      {/each}
    </Select.Group>
  </Select.Content>
</Select.Root>

<div class={`grid h-full w-full ${layout.container}`}>
  {#each slotsFor(selectedLayout) as slot (slot.position)}
    <div class={slot.classes}>
      hi
      <!-- <WidgetLayoutSlot position={slot.position} widget={null} /> -->
    </div>
  {/each}
</div>

<DragDropProvider
  {sensors}
  onDragOver={(event) => {
    const { source } = event.operation
    if (source?.type === 'layoutSlot') return
    widgets = move(widgets, event)
  }}
>
  <!-- placeholder boxes -->

  <div class="drawer">
    <Drawer.Root>
      <!-- widget layout frame -->
      <div>
        <WidgetLayoutFrame
          id="widgetFrame"
          data={meta['widgetFrame']}
          index={0}
        >
          <div class="grid h-full w-full grid-cols-2 grid-rows-2 gap-2">
            <div class="col-span-1 row-span-2 outline-2">
              <!-- <WidgetLayoutSlot widget={undefined} index={0} /> -->
              <Drawer.Trigger>
                <Button
                  variant="outline"
                  onclick={() => selectSlotId('VSplitWithRightHSplit.left')}
                  >Select</Button
                >
              </Drawer.Trigger>
            </div>
            <div class="col-span-1 row-span-1 outline-2">
              <!-- <WidgetLayoutSlot widget={undefined} index={0} /> -->
              <Drawer.Trigger>
                <Button
                  variant="outline"
                  onclick={() =>
                    selectSlotId('VSplitWithRightHSplit.right_top')}
                  >Select</Button
                >
              </Drawer.Trigger>
            </div>
            <div class="col-span-1 row-span-1 outline-2">
              <!-- <WidgetLayoutSlot widget={undefined} index={0} /> -->
              <Drawer.Trigger>
                <Button
                  variant="outline"
                  onclick={() =>
                    selectSlotId('VSplitWithRightHSplit.right_bottom')}
                  >Select</Button
                >
              </Drawer.Trigger>
            </div>
          </div>
        </WidgetLayoutFrame>
      </div>

      <Drawer.Content>
        <Drawer.Header>
          <Drawer.Title>Available Widgets:</Drawer.Title>
          <Drawer.Description>Pick One.</Drawer.Description>
        </Drawer.Header>

        <!-- widget pool -->
        <div class="flex flex-col items-center">
          {#each widgets.widgetPool as widget, widgetIdx (widget.id)}
            {@const groupId = 'widgetPool'}
            <WidgetItem
              {widget}
              index={widgetIdx}
              group={groupId}
              data={{ group: groupId }}
            />
          {/each}
        </div>

        <Drawer.Footer>
          <Button>Submit</Button>
          <Drawer.Close>Cancel</Drawer.Close>
        </Drawer.Footer>
      </Drawer.Content>
    </Drawer.Root>
  </div>

  <p class="text-sm text-center text-[#9E9E9E] font-medium pt-3">
    Drag and drop to reorder
  </p>

  <!-- where the actual content is, the cards you drag -->
  <DragOverlay>
    {#snippet children(source)}
      <!-- if has group, is item -->
      {#if source.id === 'column'}
        {@const task = widgets[source.data.group as keyof typeof widgets]?.find(
          (task) => task.id === source.id
        )!}
        <!-- <WidgetItem widget={task} index={0} isOverlay /> -->
      {:else}
        <!-- else is a column -->
        <!-- <WidgetLayoutSlot
          id={source.id}
          data={meta[source.id]}
          index={0}
          isOverlay
        >
          {#each widgets[source.id as keyof typeof widgets] as item, itemIdx (item.id)}
            <WidgetItem
              widget={item}
              index={itemIdx}
              group={source.id}
              data={{ group: source.id }}
            />
          {/each}
        </WidgetLayoutSlot> -->

        <div>
          <WidgetLayoutFrame
            id="widgetFrame"
            data={meta['widgetFrame']}
            index={0}
          >
            <div class="grid h-full w-full grid-cols-2 grid-rows-2 gap-2">
              <div class="col-span-1 row-span-2">
                <WidgetLayoutSlot widget={undefined} index={0} />
              </div>
              <div class="col-span-1 row-span-1">
                <WidgetLayoutSlot widget={undefined} index={0} />
              </div>
              <div class="col-span-1 row-span-1">
                <WidgetLayoutSlot widget={undefined} index={0} />
              </div>
            </div>
          </WidgetLayoutFrame>
        </div>
      {/if}
    {/snippet}
  </DragOverlay>
</DragDropProvider>
