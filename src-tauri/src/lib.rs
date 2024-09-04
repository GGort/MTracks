// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::fs::File;
use std::io::BufReader;
use cpal::HostId;
use rodio::{Decoder, OutputStream, Sink, cpal};

use rodio::cpal::traits::{HostTrait, DeviceTrait};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![play_test_sound, get_audio_devices, get_audio_hosts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
async fn get_audio_hosts() -> Vec<String> {
    let hosts = cpal::available_hosts();

    let mut result = Vec::new();

    for host in hosts {
        let host_name = host.name().into();
        result.push(host_name);
    }

    result
}



#[tauri::command]
async fn get_audio_devices(host: String) -> Result<Vec<String>, String> {
    // Match the input string to known HostId variants
    // Match the input string to known HostId variants
    let audio_host_id = match host.to_lowercase().as_str() {
        #[cfg(target_os = "linux")]
        "alsa" => HostId::Alsa,

        #[cfg(target_os = "linux")]
        "jack" => HostId::Jack,

        #[cfg(target_os = "windows")]
        "wasapi" => HostId::Wasapi,

        #[cfg(target_os = "windows")]
        "asio" => HostId::Asio,

        #[cfg(target_os = "macos")]
        "coreaudio" => HostId::CoreAudio,

        _ => return Err(format!("Unsupported or unavailable audio host: {}", host)),
    };

    let audio_host = cpal::host_from_id(audio_host_id)
        .map_err(|_| format!("Failed to get host for {}", host))?;
    let devices = audio_host.output_devices()
        .map_err(|_| "Failed to get output devices".to_string())?;

    let mut result = Vec::new();
    for device in devices {
        let dev_name = device.name().unwrap_or_else(|_| "Unknown Device".to_string());

        result.push(dev_name);
    }

    Ok(result)
}


#[tauri::command]
async fn play_test_sound() {
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();


    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("test/MultiTracks/Click Track.wav").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    // stream_handle.play_raw(source.convert_samples()).expect("TODO: panic message");

    sink.append(source);
    sink.sleep_until_end();

    // The sound plays in a separate audio thread,
    // so we need to keep the mai   n thread alive while it's playing.
    // std::thread::sleep(std::time::Duration::from_secs(5));
}
