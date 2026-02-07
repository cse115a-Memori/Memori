## tauri setup

```sh
bun create <app_name>
cd <app_name>
bun run tauri ios init
bun run tauri ios dev --open --host

bun tauri add geolocation

bun install svelte-gestures

bun add @mnlphlp/plugin-blec
cargo add tauri-plugin-blec

# bun tauri add store
bun add @tauri-store/svelte
cargo add tauri-plugin-svelte
```

## svelte setup

```sh
bun add -D typescript-svelte-plugin @types/node @biomejs/biome @lucide/svelte && bun add runed neverthrow

bunx sv add tailwindcss devtools-json

bun install -D prettier-plugin-svelte prettier
```

```sh
bun install framework7 framework7-svelte
```

# shadcn

```sh
bun install tailwind-variants clsx tailwind-merge tw-animate-css

bun x shadcn-svelte@latest init

bun x shadcn-svelte@latest add -a
```

## threlte

```sh
bun install three @threlte/core \
            @threlte/extras \
            @types/three
```

## taurpc

- only works on desktop

```sh
bun install taurpc

cargo add taurpc
cargo add thiserror
cargo add specta_typescript
cargo add specta@=2.0.0-rc.22 --features derive
cargo add tokio --features full
```
