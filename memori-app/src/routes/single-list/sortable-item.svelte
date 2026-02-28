<script lang="ts">
	import { type UseSortableInput, useSortable } from '@dnd-kit-svelte/svelte/sortable'
	import type { ClassValue } from 'svelte/elements'
	import { Button } from '@/components/ui/button'
	import {
		sortableCardBaseClasses,
		sortableCardContentClasses,
		sortableCardInteractiveClasses,
		sortableCardPlaceholderClasses,
		sortableCardPlaceholderTextClasses,
		sortableCardTitleClasses,
	} from './sortable-item-classes'

	interface Widget {
		id: string
		content: string
	}

	interface Props extends UseSortableInput {
		widget: Widget
		isOverlay?: boolean
		inFrame?: boolean
		cls?: ClassValue
		overlayStyle?: string
	}

	let {
		widget,
		isOverlay = false,
		inFrame,
		cls,
		overlayStyle,
		...rest
	}: Props = $props()

	// svelte-ignore state_referenced_locally
	const { ref, isDragging, isDropping } = useSortable({ ...rest, feedback: 'move' })

	$effect(() => {})
</script>

<div class={["relative select-none outline-2", cls]} style={overlayStyle} {@attach ref}>
	{#if !isDropping.current}
		<!-- overlay -->
		<div
			class={[
	      sortableCardBaseClasses,
	      sortableCardInteractiveClasses,
	      { 'transition-none': isDragging.current || isDropping.current },
	      { 'cursor-grabbing': isDragging.current || isOverlay },
	      { invisible: (isDragging.current) && !isOverlay },
	      'w-full h-full outline-2',
	    ]}
		>
			<div class={sortableCardTitleClasses}>{widget.content}</div>
			{#if widget.content}
				<p class={sortableCardContentClasses}>{widget.content}</p>
			{:else}
				<Button>Select</Button>
			{/if}
		</div>
	{/if}

	{#if !isOverlay && isDragging.current}
		<div class="absolute inset-0 flex items-center justify-center h-full w-full">
			<div class={sortableCardPlaceholderClasses}>
				<span class={sortableCardPlaceholderTextClasses}>Moving: {widget.content}</span>
				hi
			</div>
		</div>
	{/if}
</div>
