use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use keyring::Entry;
use tauri::Manager;
use crate::models::connection::DbConnection;

const KEYRING_SERVICE: &str = "tmyadmin_db_connections";

pub fn save_password(connection_id: &str, password: &str) -> Result<(), String> {
    let entry = Entry::new(KEYRING_SERVICE, connection_id)
        .map_err(|e| e.to_string())?;
    entry.set_password(password).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_password(connection_id: &str) -> Result<Option<String>, String> {
    let entry = Entry::new(KEYRING_SERVICE, connection_id)
        .map_err(|e| e.to_string())?;
    match entry.get_password() {
        Ok(pass) => Ok(Some(pass)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

pub fn delete_password(connection_id: &str) -> Result<(), String> {
    let entry = Entry::new(KEYRING_SERVICE, connection_id)
        .map_err(|e| e.to_string())?;
    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn get_connections_file_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let mut path = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    create_dir_all(&path).map_err(|e| e.to_string())?;
    path.push("connections.json");
    Ok(path)
}

pub fn read_connections(app_handle: &tauri::AppHandle) -> Result<Vec<DbConnection>, String> {
    let path = get_connections_file_path(app_handle)?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
    if contents.trim().is_empty() {
        return Ok(Vec::new());
    }
    let connections: Vec<DbConnection> = serde_json::from_str(&contents).map_err(|e| e.to_string())?;
    Ok(connections)
}

pub fn write_connections(app_handle: &tauri::AppHandle, connections: &[DbConnection]) -> Result<(), String> {
    let path = get_connections_file_path(app_handle)?;
    let contents = serde_json::to_string_pretty(connections).map_err(|e| e.to_string())?;
    let mut file = File::create(path).map_err(|e| e.to_string())?;
    file.write_all(contents.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}
