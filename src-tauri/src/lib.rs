// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod songs;
mod settings;
mod audio_devices;

use crate::songs::*;
use crate::audio_devices::*;





#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![play_test_sound, get_audio_devices, get_audio_hosts, get_songs, get_song])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

