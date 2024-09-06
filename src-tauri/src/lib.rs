/*
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
mod songs;
mod settings;
mod audio_devices;

use crate::songs::*;
use crate::audio_devices::*;
use crate::settings::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            play_test_sound,
            get_audio_devices,
            get_audio_hosts,
            get_songs,
            get_song,
            set_config_item,
            get_config_item
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

