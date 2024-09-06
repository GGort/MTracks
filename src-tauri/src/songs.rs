use std::fs;
use std::path::PathBuf;
use crate::settings::get_songs_dir;

mod song_db;
use crate::songs::song_db::*;


#[tauri::command]
pub async fn get_songs() -> Result<Vec<SongInfo>, String> {

    let mtracks_dir = get_songs_dir()?;

    // Prepare a list to hold the song information
    let mut song_info_list: Vec<SongInfo> = Vec::new();

    // Iterate over the subdirectories in /MTracks
    let entries = fs::read_dir(&mtracks_dir).map_err(|_| "Could not read the MTracks directory")?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;

        // Iterate over the subdirectories in /MTracks
        let path: PathBuf = entry.path();

        // Skip files
        if !path.is_dir() { continue; };

        let folder_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown"); // Get the folder name as a fallback

        let db_path = path.join("meta.db");

        if !db_path.exists() || !db_path.is_file() {
            song_info_list.push(SongInfo {
                title: folder_name.into(),
                ..Default::default()
            });
            continue;
        }

        // Read Database
        song_info_list.push(get_info(db_path, folder_name.into()));
    }

    // Return the list of SongInfo objects directly
    Ok(song_info_list)
}

#[tauri::command]
pub async fn get_song(title: String) -> Result<SongInfo, String> {
    let mut mtracks_dir = get_songs_dir()?;
    mtracks_dir.push(&title);

    if !mtracks_dir.exists() {
        return Err(format!("Failed to find Song: {}", title.clone()));
    }
    let db_path = mtracks_dir.join("meta.db");

    if !db_path.exists() || !db_path.is_file(){
        return Err(format!("Song Has no Info file: {}", title.clone()));
    }

    Ok(get_info(db_path, title.into()))
}