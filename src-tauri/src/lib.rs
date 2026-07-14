pub mod models {
    pub mod connection;
}
pub mod db {
    pub mod db_connector;
    pub mod secure_storage;
}

use models::connection::DbConnection;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn test_connection(conn: DbConnection, password: Option<String>) -> Result<(), String> {
    let connector = db::db_connector::create_connector(conn);
    connector.test_connection(password.as_deref()).await
}

#[tauri::command]
async fn save_connection(app_handle: tauri::AppHandle, conn: DbConnection, password: Option<String>) -> Result<(), String> {
    if let Some(ref pass) = password {
        db::secure_storage::save_password(&conn.id, pass)?;
    }
    let mut conns = db::secure_storage::read_connections(&app_handle)?;
    if let Some(idx) = conns.iter().position(|c| c.id == conn.id) {
        conns[idx] = conn;
    } else {
        conns.push(conn);
    }
    db::secure_storage::write_connections(&app_handle, &conns)?;
    Ok(())
}

#[tauri::command]
fn list_connections(app_handle: tauri::AppHandle) -> Result<Vec<DbConnection>, String> {
    db::secure_storage::read_connections(&app_handle)
}

#[tauri::command]
fn delete_connection(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let _ = db::secure_storage::delete_password(&id);
    let mut conns = db::secure_storage::read_connections(&app_handle)?;
    conns.retain(|c| c.id != id);
    db::secure_storage::write_connections(&app_handle, &conns)?;
    Ok(())
}

#[tauri::command]
fn get_connection_password(id: String) -> Result<Option<String>, String> {
    db::secure_storage::get_password(&id)
}

#[tauri::command]
async fn list_connection_databases(conn: DbConnection, password: Option<String>) -> Result<Vec<String>, String> {
    let connector = db::db_connector::create_connector(conn);
    connector.list_databases(password.as_deref()).await
}

#[tauri::command]
async fn list_connection_tables(mut conn: DbConnection, password: Option<String>, database: Option<String>) -> Result<Vec<String>, String> {
    if let Some(db_name) = database {
        conn.database = Some(db_name);
    }
    let connector = db::db_connector::create_connector(conn);
    connector.list_tables(password.as_deref()).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            test_connection,
            save_connection,
            list_connections,
            delete_connection,
            get_connection_password,
            list_connection_databases,
            list_connection_tables
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
