<script lang="ts">
	import { invoke } from '@tauri-apps/api/core'

  // shadcn-svelte
  import { Button } from '$lib/components/ui/button/index.js'
  import * as Field from '$lib/components/ui/field/index.js'
  import { Input } from '$lib/components/ui/input/index.js'
  import { cn } from '$lib/utils.js'

  let name = $state('')
  let greetMsg = $state('')

  async function greet(event: Event) {
    event.preventDefault()
    greetMsg = await invoke('greet', { name })
  }

  const cls = {
    page: cn(
      'm-0 min-h-screen pt-[10vh] flex flex-col justify-center text-center antialiased',
      'font-[Inter,_Avenir,_Helvetica,_Arial,_sans-serif] text-base leading-6 font-normal',
      'bg-[#f6f6f6] text-[#0f0f0f]',
      'dark:bg-[#2f2f2f] dark:text-[#f6f6f6]'
    ),
    row: 'flex justify-center',
    logo: 'h-24 p-6 transition duration-[750ms] will-change-[filter]',
    logoLink: cn(
      'p-0 h-auto',
      'text-[#646cff] hover:text-[#535bf2] no-underline hover:no-underline',
      'dark:hover:text-[#24c8db]'
    ),
    input: cn(
      'mr-[5px] rounded-lg border border-transparent px-5 py-2.5 text-base font-medium',
      'bg-white text-[#0f0f0f] shadow-[0_2px_2px_rgba(0,0,0,0.2)]',
      'focus-visible:ring-0 focus-visible:ring-offset-0',
      'dark:bg-[#0f0f0f98] dark:text-white'
    ),
    button: cn(
      'rounded-lg border border-transparent px-5 py-2.5 text-base font-medium',
      'bg-white text-[#0f0f0f] shadow-[0_2px_2px_rgba(0,0,0,0.2)]',
      'hover:border-[#396cd8] active:border-[#396cd8] active:bg-[#e8e8e8]',
      'focus-visible:ring-0 focus-visible:ring-offset-0',
      'dark:bg-[#0f0f0f98] dark:text-white dark:active:bg-[#0f0f0f69]'
    )
  }
</script>

<main class={cls.page}>
	<h1 class="text-3xl font-semibold">Welcome to Tauri + Svelte</h1>

	<div class={cls.row}>
		<Button
			href="https://vite.dev"
			target="_blank"
			rel="noreferrer"
			variant="link"
			class={cn(cls.logoLink, 'hover:no-underline')}
			aria-label="Learn more about Vite"
		>
			<img
				src="/vite.svg"
				alt="Vite Logo"
				class={cn(cls.logo, 'hover:drop-shadow-[0_0_2em_#747bff]')}
			/>
		</Button>

		<Button
			href="https://tauri.app"
			target="_blank"
			rel="noreferrer"
			variant="link"
			class={cn(cls.logoLink, 'hover:no-underline')}
			aria-label="Learn more about Tauri"
		>
			<img
				src="/tauri.svg"
				alt="Tauri Logo"
				class={cn(cls.logo, 'hover:drop-shadow-[0_0_2em_#24c8db]')}
			/>
		</Button>

		<Button
			href="https://svelte.dev"
			target="_blank"
			rel="noreferrer"
			variant="link"
			class={cn(cls.logoLink, 'hover:no-underline')}
			aria-label="Learn more about Svelte"
		>
			<img
				src="/svelte.svg"
				alt="SvelteKit Logo"
				class={cn(cls.logo, 'hover:drop-shadow-[0_0_2em_#ff3e00]')}
			/>
		</Button>
	</div>

	<p class="mt-2">
		Click on the Tauri, Vite, and SvelteKit logos to learn more.
	</p>

	<form class="mt-4" onsubmit={greet}>
		<Field.Field
			orientation="horizontal"
			class="justify-center mx-auto max-w-xs"
		>
			<Field.Label for="greet-input" class="sr-only">Name</Field.Label>

			<Input
				id="greet-input"
				placeholder="Enter a name..."
				bind:value={name}
				class={cls.input}
			/>

			<Button type="submit" variant="outline" class={cls.button}>
				Greet
			</Button>
		</Field.Field>
	</form>

	<p class="mt-4">{greetMsg}</p>
</main>
