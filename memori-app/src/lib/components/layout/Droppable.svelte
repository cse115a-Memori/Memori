<script lang="ts">
	import type {
		CollisionPriority,
		Data,
		Draggable,
		Type,
		UniqueIdentifier,
	} from '@dnd-kit/abstract'
	import { useDroppable } from '@dnd-kit-svelte/svelte'
	import type { Snippet } from 'svelte'
	import type { ClassValue } from 'svelte/elements'

	interface DroppableProps {
		id: UniqueIdentifier
		type?: Type
		accept?: Type | Type[] | ((source: Draggable<Data>) => boolean)
		collisionPriority?: CollisionPriority | number
		data?: Data
		disabled?: boolean
		children: Snippet
		class?: ClassValue
	}

	let {
		id,
		type,
		accept,
		collisionPriority,
		data,
		disabled,
		children,
		class: className,
	}: DroppableProps = $props()

	const { ref } = useDroppable({
		id: () => id,
		type: () => type,
		accept: () => accept,
		collisionPriority: () => collisionPriority,
		data: () => data,
		disabled: () => disabled,
	})
</script>

<div class={className} {@attach ref}>{@render children()}</div>
