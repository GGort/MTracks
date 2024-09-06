use std::fs;
use std::path::PathBuf;

pub fn get_songs_dir() -> Result<PathBuf, String>{
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