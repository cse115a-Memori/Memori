<script lang="ts">
	import { CollisionPriority } from '@dnd-kit/abstract'
	import {
		getSlotClassByIndex,
		type LayoutVariant,
	} from '@/features/widgets/model/layout'
	import type { GroupId, WidgetView } from '@/features/widgets/model/widget-frame'
	import Droppable from './Droppable.svelte'
	import SortableItem from './SortableItem.svelte'

	interface Props {
		id: GroupId
		title: string
		widgets: WidgetView[]
		layout: LayoutVariant
		frameContainerClass: string
	}

	let { id, title, widgets, layout, frameContainerClass }: Props = $props()
	const isFrameWidgets = $derived(id === 'frame-widgets')
</script>

<section class="bg-[#F9F9F9] rounded-3xl p-3 pt-6">
	<p class="text-lg font-bold pb-3">{title}</p>

	<Droppable
		class="min-h-24 rounded-3xl border border-slate-200/70 bg-white/55 p-2"
		{id}
		type="column"
		accept="item"
		collisionPriority={CollisionPriority.Low}
	>
		<div
			class={isFrameWidgets ? ['grid gap-2 h-52', frameContainerClass] : 'flex flex-col gap-2'}
		>
			{#each widgets as widget, index (widget.id)}
				<SortableItem
					{widget}
					id={widget.id}
					cls={getSlotClassByIndex(layout, index)}
					{index}
					group={id}
					data={{ group: id, index }}
					type="item"
				/>
			{/each}
		</div>
	</Droppable>
</section>
