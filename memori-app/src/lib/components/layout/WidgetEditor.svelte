<script lang="ts">
	import { onMount } from 'svelte'
	import {
		formatCompactClock,
		WidgetsEditorFrame,
		WidgetsToolbar,
	} from '@/components/layout'
	import { startAuthStore } from '@/features/auth/store'
	import { connState } from '@/features/connection'
	import { prefsState } from '@/features/prefs/store'
	import {
		selectFlashPayload,
		validateWidgetsStateForFlash,
	} from '@/features/widgets/flash'
	import {
		isLayoutVariant,
		type WidgetFrame,
	} from '@/features/widgets/model/widget-frame'
	import { getWidgetKinds } from '@/features/widgets/service'
	import type { WidgetsState } from '@/features/widgets/types'
	import {
		selectWidgetFrameEntry,
		setWidgetFrameLayout,
		syncWidgets,
		widgetsState,
	} from '@/features/widgets/widgets-store'
	import { commands, tryCmd } from '@/tauri'

	let flashErr = $state('')
	let now = $state(new Date())
	const CLOCK_TICK_MS = 1000

	let isFlashing = $state(false)
	const flash = async () => {
		const snapshot = $state.snapshot(widgetsState) as WidgetsState
		const validationErr = validateWidgetsStateForFlash(snapshot)
		if (validationErr) {
			flashErr = validationErr
			return
		}

		flashErr = ''
		isFlashing = true
		
		await loadWidgets()
		
		const payload = selectFlashPayload(snapshot)

		console.log('flashing payload', payload)

		await tryCmd(commands.flashMemoriState(payload)).match(
			() => {
				flashErr = ''
			},
			error => {
				flashErr = `Flash failed: ${error}`
			}
		)
		isFlashing = false
	}

	const loadWidgets = (): Promise<void> =>
		getWidgetKinds().match(
			widgets => {
				syncWidgets(widgets)
			},
			error => {
				flashErr = `Load widgets failed: ${error}`
			}
		)

	const compactClock = $derived(
		formatCompactClock(now, prefsState.systemOptions.timeZone ?? undefined)
	)

	onMount(() => {
		void startAuthStore().finally(() => {
			void loadWidgets()
		})

		const intervalId = setInterval(() => {
			now = new Date()
		}, CLOCK_TICK_MS)

		return () => {
			clearInterval(intervalId)
		}
	})

	const frameEntry = $derived(selectWidgetFrameEntry(widgetsState.activeFrameIdx))
	const activeLayout = $derived(frameEntry.activeLayout)
	const frameLayouts = $derived(frameEntry.frameLayouts)

	const handleLayoutChange = (nextLayout: string) => {
		if (!isLayoutVariant(nextLayout)) return
		setWidgetFrameLayout(widgetsState.activeFrameIdx, nextLayout)
	}
</script>

<WidgetsToolbar
	layout={activeLayout}
	{isFlashing}
	onLayoutChange={handleLayoutChange}
	onFlash={flash}
/>

{#if flashErr}
	<p class="text-sm text-red-600">{flashErr}</p>
{/if}

<WidgetsEditorFrame layout={activeLayout} frame={frameLayouts} {compactClock} />
