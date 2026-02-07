import { RuneStore } from '@tauri-store/svelte'

export const store = new RuneStore('ble', {
	resolvedNames: {} as Record<string, string>,
	showAllDevices: false,
})

// await store.start()
// await store.destroy()
