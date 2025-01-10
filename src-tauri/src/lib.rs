pub mod commands;
pub mod google_drive;
pub mod file_utils;

use std::path::Path;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            file_utils::create_folder(Path::new("modpacks"))?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::get_packs])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}