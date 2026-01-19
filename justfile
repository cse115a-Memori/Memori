list:
    just --list

esp LOG="debug":
    cd ./memori-dev/memori-esp32c3/ && ESP_LOG={{ LOG }} cargo run --release

ios:
    cd ./memori-app && bun tauri ios dev "iPhone 17 Pro"

sim LOG="debug":
    cd ./memori-dev/simulator && RUST_LOG={{ LOG }} cargo run --release

typ FILE="":
    typst watch {{ FILE }}

[working-directory('memori-app')]
dev:
    bun run dev --open

[working-directory('memori-app')]
check:
    bunx @biomejs/biome check --write .
    bunx dprint fmt "**/*.{svelte,astro}"
    bunx sv check --compiler-warnings "state_referenced_locally:ignore"

[working-directory('memori-app')]
shad *ARGS="":
    bunx shadcn-svelte@latest {{ ARGS }}
