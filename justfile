typ NAME="":
    typst watch {{ NAME }}

dev LOG="debug":
    cd ./memori-dev && ESP_LOG={{ LOG }} cargo run --release

ios:
    cd ./memori-app && bun tauri ios dev "iPhone 17 Pro"
