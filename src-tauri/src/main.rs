// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod parsertime;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![validate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn validate(file: String, source: String, compatibility: String) -> String {
    let request = parsertime::Request {
        file: file,
        source: source,
        compatibility: compatibility,
    };

    let result = parsertime::validate(request);
    match result {
        Ok(_) => "Validation successful".to_string(),
        Err(e) => e,
    }
}
