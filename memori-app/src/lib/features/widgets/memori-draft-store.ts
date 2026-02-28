import { RuneStore } from '@tauri-store/svelte'
import type { MemoriDraftState } from '@/features/widgets/types.ts'
import type { MemoriStateInput } from '@/tauri'

const initialMemoriDraftState: MemoriDraftState = {
	draft: null,
}

const memoriDraftStore = new RuneStore<MemoriDraftState>(
	'memori-draft',
	initialMemoriDraftState,
	{
		autoStart: false,
		saveOnChange: true,
		hooks: {
			error: error => {
				console.error('Memori draft store error:', error)
			},
		},
	}
)

export const memoriDraftState = memoriDraftStore.state

let startPromise: Promise<void> | null = null

export function setMemoriDraft(draft: MemoriStateInput): void {
	memoriDraftState.draft = draft
}

export function startMemoriDraftStore(): Promise<void> {
	startPromise ??= memoriDraftStore.start().catch(error => {
		startPromise = null
		throw error
	})

	return startPromise
}
