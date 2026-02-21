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
    bunx tauri ios dev --host --open

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

## build targets on external drive

MOUNT := "/Volumes/MemoriTarget"
EXT_TARGET := MOUNT / "memori-tauri-target"

### for exFat ssd setup
# hdiutil create -size 80g -type SPARSEBUNDLE -fs APFS -volname MemoriTarget "/Volumes/X31/MemoriTarget.sparsebundle"
# hdiutil attach "/Volumes/X31/MemoriTarget.sparsebundle"

ext +args:
    #!/usr/bin/env bash
    set -euo pipefail

    cd "memori-app"

    if [[ -d "{{ MOUNT }}" ]]; then
        export CARGO_TARGET_DIR="{{ EXT_TARGET }}"
        mkdir -p "$CARGO_TARGET_DIR"
        echo "Using external CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
    else
        echo "External drive not mounted at {{ MOUNT }}; using default ./target"
    fi

    exec just --justfile "{{ justfile() }}" {{ args }}
