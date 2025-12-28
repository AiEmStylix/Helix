use std::env;

use sqlx::sqlite::SqliteQueryResult;

use crate::sqlite::{create_connection, execute::execute_sql};
use tauri::Manager;
mod sqlite;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
