<div class="page">
	<div class="content">
		<header class="quiet-header">A gentle reminder.</header>

		<main class="center">
			<h1 class="title">MEMENTO MORI</h1>
			<div class="divider" aria-hidden="true"></div>
			<p class="subtext">Let this make you present.</p>

			<label class="prompt-label" for="intention">One thing that matters today</label>
			<input
				id="intention"
				class="prompt-input"
				type="text"
				placeholder="Write it softly..."
			/>

			<button class="primary-btn" type="button">Continue</button>
		</main>

		<footer class="footer">This page is not a warning. It's an invitation.</footer>
	</div>
</div>

<style>
	:global(body) {
		margin: 0;
	}

	.page {
		position: relative;
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: clamp(2.5rem, 6vw, 5rem);
		background: var(--background);
		color: var(--foreground);
		overflow: hidden;
	}

	.page::before {
		content: '';
		position: absolute;
		inset: -20%;
		background:
			radial-gradient(60% 50% at 50% 0%, oklch(0.99 0.01 85 / 0.55) 0%, transparent 70%),
			radial-gradient(
				40% 35% at 85% 10%,
				oklch(0.98 0.02 85 / 0.35) 0%,
				transparent 60%
			);
		pointer-events: none;
	}

	.page::after {
		content: '';
		position: absolute;
		inset: 0;
		background-image: repeating-linear-gradient(
			0deg,
			oklch(0.8 0.02 85 / 0.03) 0,
			oklch(0.8 0.02 85 / 0.03) 1px,
			transparent 1px,
			transparent 3px
		);
		opacity: 0.35;
		pointer-events: none;
	}

	.content {
		position: relative;
		z-index: 1;
		width: min(42rem, 90vw);
		text-align: center;
		display: grid;
		gap: 2.5rem;
	}

	.content::before {
		content: '';
		position: absolute;
		width: min(34rem, 75vw);
		aspect-ratio: 1;
		border: 1px solid oklch(0.7 0.04 85 / 0.12);
		border-radius: 999px;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -55%);
		pointer-events: none;
	}

	.quiet-header {
		font-size: 0.85rem;
		letter-spacing: 0.08em;
		text-transform: uppercase;
		color: var(--muted-foreground);
	}

	.center {
		display: grid;
		justify-items: center;
		gap: 1.25rem;
	}

	.title {
		font-family: var(--font-serif);
		font-size: clamp(2.75rem, 6vw, 4.25rem);
		letter-spacing: 0.08em;
		font-weight: 500;
		margin: 0;
	}

	.divider {
		width: min(10rem, 40vw);
		height: 1px;
		background: oklch(0.7 0.02 85 / 0.3);
	}

	.subtext {
		margin: 0;
		color: var(--muted-foreground);
		font-size: 1.05rem;
	}

	.prompt-label {
		font-size: 0.95rem;
		color: var(--muted-foreground);
	}

	.prompt-input {
		width: min(22rem, 80vw);
		padding: 0.75rem 1rem;
		border-radius: 0.6rem;
		border: 1px solid var(--border);
		background: var(--card);
		color: var(--foreground);
		font-size: 1rem;
		outline: none;
		transition:
			border-color 200ms ease,
			box-shadow 200ms ease;
	}

	.prompt-input:focus {
		border-color: var(--ring);
		box-shadow: 0 0 0 3px oklch(0.7 0.06 85 / 0.25);
	}

	.primary-btn {
		margin-top: 0.5rem;
		padding: 0.7rem 2.2rem;
		border-radius: 999px;
		border: 1px solid var(--border);
		background: transparent;
		color: var(--foreground);
		font-size: 0.95rem;
		letter-spacing: 0.04em;
		text-transform: uppercase;
		transition:
			border-color 200ms ease,
			color 200ms ease,
			background 200ms ease;
		cursor: pointer;
	}

	.primary-btn:hover {
		border-color: var(--ring);
		background: oklch(0.94 0.02 85 / 0.6);
	}

	.footer {
		font-size: 0.8rem;
		color: var(--muted-foreground);
	}
</style>
