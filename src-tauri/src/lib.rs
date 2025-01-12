pub mod commands;
pub mod google_drive;
pub mod file_utils;
pub mod modpack_processor;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::modpacks_load])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}