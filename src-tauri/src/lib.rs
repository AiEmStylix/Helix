use std::env;

use crate::{
    commands::sqlite::{save_sqlite_connection, sqlite_list_connections},
    sqlite::{create_connection, execute::execute_sql},
};

use tauri::Manager;

mod commands;
mod sqlite;
mod storage;

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
            sqlite_list_connections,
            save_sqlite_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
