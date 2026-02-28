<script lang="ts">
	import { LoaderCircle } from '@lucide/svelte'
	import { Button } from '@/components/ui/button/index.ts'
	import { LAYOUT_VARIANTS, type LayoutVariant } from '@/model/layout.ts'
	import * as Select from '$lib/components/ui/select/index.js'

	interface Props {
		layout: LayoutVariant
		isFlashing: boolean
		onLayoutChange: (nextLayout: string) => void
		onFlash: () => void
	}

	let {
		layout,
		isFlashing: isSettingWidget,
		onLayoutChange,
		onFlash: setWidget,
	}: Props = $props()
</script>

<div class="flex justify-between">
	<Select.Root
		type="single"
		name="layoutSelector"
		value={layout}
		disabled={isSettingWidget}
		onValueChange={onLayoutChange}
	>
		<Select.Trigger class="">{layout}</Select.Trigger>
		<Select.Content>
			<Select.Group>
				<Select.Label>Layouts</Select.Label>
				{#each LAYOUT_VARIANTS as layoutVariant (layoutVariant)}
					<Select.Item value={layoutVariant} label={layoutVariant}>
						{layoutVariant}
					</Select.Item>
				{/each}
			</Select.Group>
		</Select.Content>
	</Select.Root>

	<Button onclick={setWidget}>
		{#if isSettingWidget}
			<div class="animate-spin"><LoaderCircle /></div>
		{:else}
			Update
		{/if}
	</Button>
</div>
