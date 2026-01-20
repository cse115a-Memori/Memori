[working-directory: 'memori-dev/memori-esp32c3']
esp LOG="debug":
    ESP_LOG={{ LOG }} cargo run --release

[working-directory: 'memori-dev/simulator']
memsim LOG="debug":
    RUST_LOG={{ LOG }} cargo run --release

[working-directory: 'memori-app']
ios-sim:
    bun tauri ios dev "iPhone 17 Pro"

[working-directory: 'memori-app']
desktop:
    bun tauri dev 
    
typ FILE="":
    typst watch {{ FILE }}
