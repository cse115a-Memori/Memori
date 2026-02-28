<script lang="ts">
	import {
		kindToDisplay,
		type WidgetView,
	} from '@/features/widgets/model/widget-frame.ts'
	import {
		sortableCardBaseClasses,
		sortableCardContentClasses,
		sortableCardTitleClasses,
	} from './sortable-item-classes.ts'
	import type { CompactClock } from './widget-clock.ts'

	interface Props {
		activeWidget: WidgetView | null
		overlayStyle?: string
		compactClock: CompactClock
	}

	let { activeWidget, overlayStyle, compactClock }: Props = $props()
</script>

{#if activeWidget}
	{@const display = kindToDisplay(activeWidget.kind)}
	<div class="relative select-none" style={overlayStyle}>
		<div
			class={[
				sortableCardBaseClasses,
				'h-full shadow-lg ring-2 ring-sky-300/60',
			]}
		>
			<div class={sortableCardTitleClasses}>{display.name}</div>
			{#if 'Clock' in activeWidget.kind}
				<p class="text-sm font-semibold tabular-nums text-slate-700">
					{compactClock.time}
				</p>
				<p class="text-xs text-slate-500">{compactClock.zone}</p>
			{:else}
				<p class={sortableCardContentClasses}>{display.content}</p>
			{/if}
		</div>
	</div>
{/if}
