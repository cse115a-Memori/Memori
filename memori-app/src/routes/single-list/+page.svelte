<script lang="ts">
	import { CollisionPriority } from '@dnd-kit/abstract'
	import { swap } from '@dnd-kit/helpers'
	import { DragDropProvider, DragOverlay } from '@dnd-kit-svelte/svelte'
	import { RestrictToWindowEdges } from '@dnd-kit-svelte/svelte/modifiers'
	import { getOverlaySize } from '@/model'
	import { LAYOUT_TEMPLATES, type LayoutVariant } from '@/model/layout.ts'
	import { sensors } from '$lib'
	import Droppable from './droppable.svelte'
	import SortableItem from './sortable-item.svelte'

	interface Todo {
		id: string
		content: string
	}

	const poolWidgets = {
		widgets: [
			{ id: 'task-1', content: 'Learn Svelte' },
			{ id: 'task-2', content: 'Build a Kanban board' },
			{ id: 'task-3', content: 'Review code' },
			{ id: 'task-4', content: 'Setup project' },
			{ id: 'task-5', content: 'Randm thing' },
		],
	}

	const generateMockItems = (len: number) => {
		const temp: Todo[] = []
		for (let i = 0; i < len; i++) {
			temp.push({
				id: `mock-${i}`,
				content: '',
			})
		}
		return temp
	}

	const layoutVar: LayoutVariant = 'VSplitWithRightHSplit'
	const layoutCls = LAYOUT_TEMPLATES[layoutVar]
	const LIST_ID = 'widgets' as const

	type Todos = { widgets: Todo[] }
	let todos = $state<Todos>({
		[LIST_ID]: poolWidgets.widgets.concat(generateMockItems(3)),
	})

	let overlaySize = $state<{ width: number; height: number } | null>(null)
	const overlayStyle = $derived(
		overlaySize
			? `width: ${overlaySize.width}px; height: ${overlaySize.height}px;`
			: undefined
	)

	let dragSnapshot: Todo[] | null = null
	let didMutateDuringDrag = false

	$inspect(todos)
</script>

<DragDropProvider
	{sensors}
	modifiers={[RestrictToWindowEdges]}
	onDragStart={({operation}) => {
		dragSnapshot = [...todos.widgets]
		didMutateDuringDrag = false
		overlaySize = getOverlaySize(operation)
	}}
	onDragOver={(event) => {
		const source = event.operation.source
		const target = event.operation.target

		if (!source) return

		overlaySize = getOverlaySize(event.operation)

		const isSourceItem = todos.widgets.some((todo) => todo.id === source.id)
		const hasItemTarget =
			!!target && todos.widgets.some((todo) => todo.id === target.id)
		const isListTarget = target?.id === LIST_ID
		const hasValidTarget = hasItemTarget || isListTarget

		if (!isSourceItem || !hasValidTarget) {
			if (didMutateDuringDrag && dragSnapshot) {
				todos = { ...todos, widgets: [...dragSnapshot] }
				didMutateDuringDrag = false
			}
			return
		}

		if (!hasItemTarget || target.id === source.id) return

		const next = swap(todos, event)
		if (next !== todos) {
			todos = next
			didMutateDuringDrag = true
		}
	}}
	onDragEnd={(event) => {
		const { source, target, canceled } = event.operation
		const isSourceItem =
			!!source && todos.widgets.some((todo) => todo.id === source.id)
		const hasItemTarget =
			!!target && todos.widgets.some((todo) => todo.id === target.id)
		const isListTarget = target?.id === LIST_ID
		const isValidDrop =
			!canceled &&
			isSourceItem &&
			(hasItemTarget || isListTarget)

		if (!isValidDrop && dragSnapshot) {
			todos = { ...todos, widgets: [...dragSnapshot] }
		} else if (hasItemTarget && !didMutateDuringDrag) {
			todos = swap(todos, event)
		}

		dragSnapshot = null
		didMutateDuringDrag = false

		overlaySize = null
	}}
>
	{@render editor(LIST_ID, 'In Progress', todos[LIST_ID])}

	<DragOverlay>
		{#snippet children(source)}
			{@const widget = todos.widgets.find((todo) => todo.id === source.id)!}
			<SortableItem id={widget.id} {widget} index={0} isOverlay {overlayStyle} />
		{/snippet}
	</DragOverlay>
</DragDropProvider>

{#snippet editor(id: string, title: string, widgets: Todo[])}
	{@const split =  widgets.length - layoutCls.slots.length}
	{@const sideTasks = widgets.slice(0, split)}
	{@const mainTasks = widgets.slice(split, widgets.length)}
	<Droppable
		class="bg-[#F9F9F9] rounded-3xl p-3 pt-6 w-full outline-2"
		{id}
		type="column"
		accept="item"
		collisionPriority={CollisionPriority.Low}
	>
		<div class="flex gap-2">
			<section class="w-36">
				<p class="text-lg font-bold pb-3">Widgets</p>
				<div class={["grid"]}>
					{#each sideTasks as widget, index (widget.id)}
						<SortableItem
							{widget}
							id={widget.id}
							index={() => index}
							cls={widget.id.includes('mock')
									? 'hidden pointer-events-none'
									: undefined}
							type="item"
						/>
					{/each}
				</div>
			</section>
			<section class="w-full">
				<p class="text-lg font-bold pb-3">Frame</p>
				<div class={["grid gap-2 h-52", layoutCls.container]}>
					{#each mainTasks as widget, index (widget.id)}
						<SortableItem
							{widget}
							id={widget.id}
							cls={layoutCls.slots[index].classes}
							inFrame
							index={() => index + split}
							type="item"
						/>
					{/each}
				</div>
			</section>
		</div>
	</Droppable>
{/snippet}
