use std::env;

use crate::sqlite::{create_connection, execute::execute_sql};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

mod sqlite;
mod state;

#[derive(Serialize, Deserialize, Debug)]
struct DbConnection {
    name: String,
    path: String,
    created_at: DateTime<Utc>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn query(path: &str, sql: &str) -> Result<u64, String> {
    let pool = create_connection(path).await.map_err(|e| e.to_string())?;

    let result = execute_sql(sql, pool).await.map_err(|e| e.to_string())?;

    Ok(result.rows_affected())
}

#[tauri::command]
async fn new_connection(
    app: tauri::AppHandle,
    parent_window: tauri::WebviewWindow,
) -> tauri::Result<()> {
    let child_label = "connect-dialog";
    if let Some(w) = app.get_webview_window(child_label) {
        let _ = w.set_focus();
        return Ok(());
    }

    let builder = tauri::WebviewWindowBuilder::new(
        &app,
        child_label,
        tauri::WebviewUrl::App("#/connect".into()),
    )
    .title("Connect to Database")
    .inner_size(800.0, 800.0);

    let builder = builder.parent(&parent_window)?;
    builder.build()?;

    Ok(())
}

#[tauri::command]
async fn save_connection(
    app: AppHandle,
    db_name: &str,
    path: &str,
) -> tauri_plugin_store::Result<()> {
    let store = app.store("connections.json")?;

    let mut connections: Vec<DbConnection> = store
        .get("connections")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    connections.push(DbConnection {
        name: db_name.to_string(),
        path: path.to_string(),
        created_at: Utc::now(),
    });

    store.set("connections", serde_json::to_value(connections)?);

    Ok(())
}

#[tauri::command]
async fn get_connection(app: AppHandle) -> tauri_plugin_store::Result<Vec<DbConnection>> {
    let store = app.store("connections.json")?;
    let connections = store.get("connections");
    let connections = connections
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    Ok(connections)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            query,
            new_connection,
            save_connection,
            get_connection
        ])
        .setup(|app| {
            app.store("connections.json")?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
