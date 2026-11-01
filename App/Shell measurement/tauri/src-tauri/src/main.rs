#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// The single no-op typed binding.
#[tauri::command]
fn ping(text: String) -> String {
    text
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
