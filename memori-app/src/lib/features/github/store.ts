import { RuneStore } from '@tauri-store/svelte'
import type { Github } from '@/tauri'

type GitHub = Github & { init: boolean }

export type { GitHub }

const initialGitHubState: GitHub = {
	init: false,

	username: '',
	repo: '',
	openIssues: 0,
	openPrs: 0,
	stars: 0,
	notifications: 0,
	commits: [0, 0, 0, 0, 0, 0, 0],
	weekday: 0,
}

const githubStore = new RuneStore<GitHub>('github', initialGitHubState, {
	autoStart: false,
	saveOnChange: true,
	hooks: {
		error: error => {
			console.error('GitHub store error:', error)
		},
	},
})

export const githubState = githubStore.state

let startPromise: Promise<void> | null = null

export function startGitHubStore(): Promise<void> {
	startPromise ??= githubStore.start().catch(error => {
		startPromise = null
		throw error
	})

	return startPromise
}
