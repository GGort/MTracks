// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, cpal};

use rodio::cpal::traits::{HostTrait, DeviceTrait};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![play_test_sound, get_audio_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


    #[tauri::command]
    async fn get_audio_devices() -> Vec<String> {
        let hosts = cpal::available_hosts();

        for host in hosts {
            let host_name = host.name();
            println!("# Found Host: {}", host_name);
        }

        let host = cpal::default_host();
        let devices = host.output_devices().unwrap();

        let mut result = Vec::new();

        for device in devices {
            let dev_name = device.name().unwrap();

            result.push(dev_name);
        }

        result
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
