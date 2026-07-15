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

#[tauri::command]
async fn execute_connection_query(mut conn: DbConnection, password: Option<String>, query: String, database: Option<String>) -> Result<String, String> {
    if let Some(db_name) = database {
        conn.database = Some(db_name);
    }
    let connector = db::db_connector::create_connector(conn);
    connector.execute_query(&query, password.as_deref()).await
}

#[tauri::command]
async fn export_database(mut conn: DbConnection, password: Option<String>, database: String, format: String) -> Result<String, String> {
    conn.database = Some(database.clone());
    let connector = db::db_connector::create_connector(conn.clone());
    let tables = connector.list_tables(password.as_deref()).await?;
    let mut sql_dump = String::new();
    sql_dump.push_str(&format!("-- Tableflow Dump\n-- Database: {}\n-- Format: {}\n\n", database, format));
    
    if format.to_lowercase() == "sql" {
        for table in tables {
            sql_dump.push_str(&format!("-- Structure for table `{}`\n", table));
            if conn.driver == crate::models::connection::DbDriver::Mysql {
                let create_query = format!("SHOW CREATE TABLE `{}`", table);
                if let Ok(res_json) = connector.execute_query(&create_query, password.as_deref()).await {
                    if let Ok(rows) = serde_json::from_str::<serde_json::Value>(&res_json) {
                        if let Some(create_sql) = rows.get(0).and_then(|r| r.get("Create Table")).and_then(|v| v.as_str()) {
                            sql_dump.push_str(&format!("{};\n\n", create_sql));
                        }
                    }
                }
            } else {
                sql_dump.push_str(&format!("CREATE TABLE `{}` (\n  -- Columns layout\n);\n\n", table));
            }
            
            let select_query = format!("SELECT * FROM `{}` LIMIT 100", table);
            if let Ok(res_json) = connector.execute_query(&select_query, password.as_deref()).await {
                if let Ok(rows) = serde_json::from_str::<Vec<serde_json::Map<String, serde_json::Value>>>(&res_json) {
                    for row in rows {
                        let keys: Vec<String> = row.keys().map(|k| format!("`{}`", k)).collect();
                        let vals: Vec<String> = row.values().map(|v| {
                            if v.is_null() {
                                "NULL".to_string()
                            } else if v.is_string() {
                                format!("'{}'", v.as_str().unwrap().replace("'", "''"))
                            } else {
                                v.to_string()
                            }
                        }).collect();
                        if !keys.is_empty() {
                            sql_dump.push_str(&format!("INSERT INTO `{}` ({}) VALUES ({});\n", table, keys.join(", "), vals.join(", ")));
                        }
                    }
                    sql_dump.push_str("\n");
                }
            }
        }
    } else {
        sql_dump.push_str("{\n");
        for (i, table) in tables.iter().enumerate() {
            let select_query = format!("SELECT * FROM `{}` LIMIT 100", table);
            if let Ok(res_json) = connector.execute_query(&select_query, password.as_deref()).await {
                sql_dump.push_str(&format!("  \"{}\": {}{}\n", table, res_json, if i == tables.len() - 1 { "" } else { "," }));
            }
        }
        sql_dump.push_str("}\n");
    }
    
    Ok(sql_dump)
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
            list_connection_tables,
            execute_connection_query,
            export_database
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
