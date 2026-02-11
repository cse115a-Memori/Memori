list:
    just --list

# memori

[working-directory('memori-dev/memori-esp32c3')]
esp LOG="info":
    ESP_LOG={{ LOG }} cargo run --release --bin memori-esp32c3

[working-directory('memori-dev/simulator')]
memsim LOG="debug":
    RUST_LOG={{ LOG }} cargo run --release

[working-directory('memori-app')]
ios-sim:
    bun tauri ios dev "iPhone 17 Pro"

[working-directory('memori-app')]
android-sim:
    bun tauri android dev

[working-directory('memori-app')]
desktop:
    bun install
    bun tauri dev 

typ FILE="":
    typst watch {{ FILE }}

doc PATH:
    cargo doc --open {{ PATH }}

# desktop app
[working-directory('memori-app')]
pc:
    bunx tauri dev

# mobile app
[working-directory('memori-app')]
app:
    bunx tauri ios dev --host

[working-directory('memori-app')]
check:
    bunx @biomejs/biome check --write .
    # bunx dprint fmt "**/*.{svelte,astro}"
    bun run prettier --write "**/*.{svelte,astro}"
    bunx sv check --compiler-warnings "state_referenced_locally:ignore"
    cd ./src-tauri && cargo check

[working-directory('memori-app')]
shad *ARGS="":
    bunx shadcn-svelte@latest {{ ARGS }}
