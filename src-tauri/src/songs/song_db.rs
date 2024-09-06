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

use std::collections::HashMap;
use std::path::PathBuf;
use rusqlite::Connection;
use serde::Serialize;

// Define the SongInfo struct that will hold song data
#[derive(Serialize)]
pub struct SongInfo {
    pub title: String,
    pub setup: bool,
    pub artist: String,
    pub bpm: u16,
    pub meter: String,
    pub key: String,
}
impl Default for SongInfo {
    fn default() -> SongInfo {
        SongInfo {
            title: "Unknown".into(),
            setup: false,
            artist: "Unknown".into(),
            bpm: 0,
            meter: "Unknown".into(),
            key: "Unknown".into(),
        }
    }
}


pub fn get_info(db_location: PathBuf, title: String) -> SongInfo {
    let conn = match Connection::open(db_location) {
        Ok(c) => c,
        Err(_) => {
            return SongInfo {
                title: title.into(),
                ..Default::default()
            }
        }
    };


    // Create a SongInfo struct and set the title to the folder name
    let mut song_info = SongInfo {
        title: title.into(),
        setup: true, // Assuming you mark it as setup when database is found
        ..Default::default()
    };

    // Prepare the query to retrieve the artist, bpm, meter, and key
    let mut stmt = match conn.prepare("SELECT key, value FROM info WHERE key IN ('artist', 'bpm', 'meter', 'key')") {
        Ok(s) => s,
        Err(_) => {
            // Return default SongInfo if the query preparation fails
            return song_info;
        }
    };

    // Execute the query and collect results into a HashMap
    let mut key_value_map: HashMap<String, String> = HashMap::new();
    let rows = match stmt.query_map([], |row| {
        let key: String = row.get(0)?;
        let value: String = row.get(1)?;
        Ok((key, value))
    }) {
        Ok(r) => r,
        Err(_) => {
            // Return default SongInfo if the query execution fails
            return song_info;
        }
    };
    // Fill the key_value_map with results
    for row in rows {
        if let Ok((key, value)) = row {
            key_value_map.insert(key, value);
        }
    }

    // Extract the values for artist, bpm, meter, and key
    song_info.artist = key_value_map.get("artist").cloned().unwrap_or_default(); // Default to empty string
    song_info.meter = key_value_map.get("meter").cloned().unwrap_or_default();   // Default to empty string
    song_info.key = key_value_map.get("key").cloned().unwrap_or_default();       // Default to empty string
    song_info.bpm = key_value_map.get("bpm")
        .and_then(|bpm_str| bpm_str.parse::<u16>().ok())
        .unwrap_or(0); // Default to 0 if bpm is missing or can't be parsed

    song_info
}