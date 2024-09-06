use std::fs;
use std::path::PathBuf;
use rusqlite::Connection;

#[tauri::command]
pub fn get_config_item() -> Result<String, String> {
    let conn = match get_config_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err("Database Corrupt/missing".into())
    };


    Ok("".into())
}

#[tauri::command]
pub fn set_config_item(value: String) -> Result<bool, String> {
    Ok(false)
}

pub fn get_config_db_conn() -> Result<Connection, String> {
    let mut db_location = match get_conf_dir() {
        Ok(dir) => dir,
        Err(_) => return Err("Failed to get database".into())
    };

    db_location.push("config.db");

    let conn = match Connection::open(db_location) {
        Ok(c) => c,
        Err(_) => return Err("Connection Failed".into())
    };

    Ok(conn)
}

pub fn get_conf_dir() -> Result<PathBuf, String> {
    let conf_dir = match dirs_next::config_dir() {
        Some(dir) => dir,
        None => return Err("Could not locate the ConfigDir".into()),
    };

    let mut mtracks_conf_dir = conf_dir;
    mtracks_conf_dir.push("mtracks");

    if !mtracks_conf_dir.exists() {
        if let Err(e) = fs::create_dir_all(&mtracks_conf_dir) {
            return Err(format!("Failed to create MTracks config directory: {}", e));
        }
    }

    Ok(mtracks_conf_dir)
}

pub fn get_songs_dir() -> Result<PathBuf, String> {
    // Get the user's audio directory
    let audio_dir = match dirs_next::audio_dir() {
        Some(dir) => dir,
        None => return Err("Could not locate the audio directory".into()),
    };

    // Append /MTracks to the audio directory path
    let mut mtracks_dir = audio_dir;
    mtracks_dir.push("MTracks");

    // Check if the directory exists, if not, create it
    if !mtracks_dir.exists() {
        if let Err(e) = fs::create_dir_all(&mtracks_dir) {
            return Err(format!("Failed to create MTracks directory: {}", e));
        }
    }
    Ok(mtracks_dir)
}