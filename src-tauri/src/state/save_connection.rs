use serde_json::json;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

pub fn save_connection(app: AppHandle, path: &str) -> tauri_plugin_store::Result<()> {
    let store = app.store("connections.json")?;
    let connections = store
        .get("connections")
        .expect("Failed to get value from store");

    println!("{}", connections);
    store.set("connections", json!({"path": path}));
    Ok(())
}
