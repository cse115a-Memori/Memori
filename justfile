esp LOG="debug":
    cd ./memori-dev/esp32c3/ && ESP_LOG={{ LOG }} cargo run --release

ios:
    cd ./memori-app && bun tauri ios dev "iPhone 17 Pro"

typ FILE="":
    typst watch {{ FILE }}
