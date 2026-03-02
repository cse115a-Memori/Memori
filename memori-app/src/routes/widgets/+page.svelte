<script lang="ts">
	import { onMount } from 'svelte'
	import {
		formatCompactClock,
		WidgetsEditorSurface,
		WidgetsToolbar,
	} from '@/components/layout'
	import { startAuthStore } from '@/features/auth/store.ts'
	import { prefsState } from '@/features/prefs/store.ts'
	import {
		selectFlashPayload,
		validateWidgetsStateForFlash,
	} from '@/features/widgets/flash.ts'
	import {
		isLayoutVariant,
		type WidgetFrame,
	} from '@/features/widgets/model/widget-frame.ts'
	import { getWidgetKinds } from '@/features/widgets/service.ts'
	import type { WidgetsState } from '@/features/widgets/types.ts'
	import {
		selectWidgetFrameEntry,
		setWidgetFrame,
		setWidgetFrameLayout,
		syncWidgets,
		widgetsState,
	} from '@/features/widgets/widgets-store.ts'
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
		const payload = selectFlashPayload(snapshot)

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

	function handleFrameCommit(nextFrame: WidgetFrame): void {
		setWidgetFrame(widgetsState.activeFrameIdx, nextFrame)
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

<WidgetsEditorSurface
	layout={activeLayout}
	frame={frameLayouts}
	{compactClock}
	onFrameCommit={handleFrameCommit}
/>
