pub mod google_drive;
pub mod file_utils;
pub mod modpack_processor;
mod commands;

use crate::commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![modpacks_load, 
                                                get_packs, 
                                                launch_pack, 
                                                open_folder, 
                                                get_file_count, 
                                                get_icon_base64,
                                                get_nickname,
                                                set_nickname])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}