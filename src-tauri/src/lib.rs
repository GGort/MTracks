// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn play_test_sound(){
    println!("I was invoked from JS!");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![play_test_sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
