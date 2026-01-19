esp LOG="debug":
    cd ./memori-dev/memori-esp32c3/ && ESP_LOG={{ LOG }} cargo run --release

ios:
    cd ./memori-app && bun tauri ios dev "iPhone 17 Pro"

sim LOG="debug":
    cd ./memori-dev/simulator && RUST_LOG={{ LOG }} cargo run --release


typ FILE="":
    typst watch {{ FILE }}
