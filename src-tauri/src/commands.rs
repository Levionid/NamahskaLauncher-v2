use crate::google_drive;
use crate::file_utils;
use crate::modpack_processor;
use std::path::Path;
use std::fs;
use tauri::Emitter;

#[tauri::command]
pub async fn modpacks_load(window: tauri::Window) -> Result<(), String> {
    let local_folder = Path::new("../modpacks");
    file_utils::create_folder(&local_folder).map_err(|e| e.to_string())?;

    google_drive::download_versions_json(&local_folder).map_err(|e| e.to_string())?;

    let folders = google_drive::list_files_in_folder("").map_err(|e| e.to_string())?;
    let total_folders = folders.len();
    let mut processed_count = 0;

    for folder in folders {
        if folder.mime_type != "application/vnd.google-apps.folder" {
            continue;
        }
        file_utils::create_folder(&local_folder.join(folder.name.clone())).map_err(|e| e.to_string())?;
        let files = google_drive::list_files_in_folder(&folder.id).map_err(|e| e.to_string())?;

        for file in files {
            if file.name != "pack.zip" {
                let result = google_drive::download_file(&file.id, &file.name, &local_folder.join(folder.name.clone()));
                if result.is_ok() {
                    println!("Successfully downloaded file: {}", file.name);
                    processed_count += 1;
                    let progress = (processed_count as f64 / total_folders as f64 * 100.0).round() as u32;

                    window.emit(
                    "progress",
                    serde_json::json!({
                        "packName": folder.name,
                        "progress": progress
                    }),
                    ).map_err(|e| e.to_string())?;
                } else {
                    eprintln!("Error downloading file: {}. Error: {}", file.name, result.unwrap_err());
                }
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub fn modpack_load(window: tauri::Window, id: String) -> Result<(), String> {
    let local_folder = Path::new("modpacks");
    file_utils::create_folder(local_folder).map_err(|e| e.to_string())?;

    // TODO: Implement modpack_load command
    
    Ok(())
}