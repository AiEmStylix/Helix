use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(Serialize, Deserialize, Debug)]
pub struct DbConnection {
    name: String,
    path: String,
    created_at: DateTime<Utc>,
}

#[allow(dead_code)]
impl DbConnection {
    pub fn new(name: String, path: String) -> Self {
        Self {
            name,
            path,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

pub fn save_connection(app: &AppHandle, conn: DbConnection) -> tauri_plugin_store::Result<()> {
    let store = app.store("connections.json")?;
    let mut connections: Vec<DbConnection> = store
        .get("connections")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    connections.push(conn);

    store.set("connections", serde_json::to_value(connections)?);
    Ok(())
}

pub fn get_connections(app: &AppHandle) -> tauri_plugin_store::Result<Vec<DbConnection>> {
    let store = app.store("connections.json")?;
    let connections = store.get("connections");
    let connections = connections
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    Ok(connections)
}
