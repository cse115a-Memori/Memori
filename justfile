typ NAME="":
    typst watch {{ NAME }}

dev:
    cd ./memori-dev && cargo run --release

ios:
    cd ./memori-app && bun tauri ios dev "iPhone 17 Pro"
