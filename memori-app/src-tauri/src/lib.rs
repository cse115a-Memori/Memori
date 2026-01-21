#[taurpc::procedures(event_trigger = ApiEventTrigger, export_to = "../src/lib/bindings.ts")]
trait Api {
    // #[taurpc(event)]
    async fn hello(name: String) -> Result<String, String>;
}

#[derive(Clone)]
struct ApiImpl;

#[taurpc::resolvers]
impl Api for ApiImpl {
    async fn hello(self, name: String) -> Result<String, String> {
        Ok(format!("hi there, {}", name))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(taurpc::create_ipc_handler(ApiImpl.into_handler()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
