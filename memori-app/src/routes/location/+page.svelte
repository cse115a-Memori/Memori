<!-- <script lang="ts">
	import { onMount } from 'svelte'
	import { Button } from '@/components/ui/button'
	import { refreshLocationState, requestLocationState } from '@/features/prefs/service'
	import { prefsState, startPrefsStore } from '@/features/prefs/store'

	let errMsg = $state('')
	let isRequesting = $state(false)
	let hasCheckedPerm = $state(false)
	const locationStatus = $derived(prefsState.locationStatus)
	const lastKnownPos = $derived(prefsState.lastKnownLocation)
	const canEnableLocation = $derived(
		hasCheckedPerm &&
			(locationStatus === 'prompt' || locationStatus === 'prompt-with-rationale')
	)

	onMount(() => {
		void (async () => {
			try {
				await startPrefsStore()
				await refreshLocationState()
			} catch (error) {
				errMsg =
					typeof error === 'string'
						? error
						: error instanceof Error
							? error.message
							: String(error)
			} finally {
				hasCheckedPerm = true
			}
		})()
	})

	async function enableLocation() {
		isRequesting = true
		errMsg = ''
		try {
			await requestLocationState()
		} catch (error) {
			errMsg =
				typeof error === 'string'
					? error
					: error instanceof Error
						? error.message
						: String(error)
		} finally {
			isRequesting = false
		}
	}
</script> -->