<script lang="ts">
	import { kindToDisplay, type WidgetView } from '@/features/widgets/model/widget-frame'
	import { cardCls } from './sortable-item-classes'
	import type { CompactClock } from './widget-clock'

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
				cardCls.Base,
				'h-full shadow-lg ring-2 ring-sky-300/60',
			]}
		>
			<div class={cardCls.Title}>{display.name}</div>
			{#if 'Clock' in activeWidget.kind}
				<p class="text-sm font-semibold tabular-nums text-slate-700">
					{compactClock.time}
				</p>
				<p class="text-xs text-slate-500">{compactClock.zone}</p>
			{:else}
				<p class={cardCls.Content}>{display.content}</p>
			{/if}
		</div>
	</div>
{/if}
