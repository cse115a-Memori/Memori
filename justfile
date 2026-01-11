esp:
typ NAME="":
    typst watch {{ NAME }}

esp:
typ NAME="":
    typst watch {{ NAME }}

dev:
    cd ./memori-dev && cargo run --release

ios:
    cd ./memori-app && bun tauri ios dev "iPhone 17 Pro"

typ FILE="":
    typst watch {{ FILE }}

jj_init:
    jj git init --colocate
    jj bookmark track main@origin
    jj git fetch
