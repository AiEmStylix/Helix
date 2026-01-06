use tauri::AppHandle;
use tauri_plugin_store::Result;

use crate::storage::connection::{get_connections, save_connection, DbConnection};

#[tauri::command]
pub fn save_sqlite_connection(app: AppHandle, name: String, path: String) -> Result<()> {
    let connection = DbConnection::new(name, path);
    save_connection(&app, connection)
}

#[tauri::command]
pub fn sqlite_list_connections(app: AppHandle) -> Result<Vec<DbConnection>> {
    get_connections(&app)
}
