use crate::google_drive;
use crate::file_utils;
use crate::modpack_processor;
use std::path::Path;
use std::fs;
use tauri::Emitter;

#[tauri::command]
pub fn modpacks_load(window: tauri::Window) -> Result<(), Box<dyn std::error::Error>> {
    let local_folder = Path::new("modpacks");
    file_utils::create_folder(local_folder)?;

    let versions_path = local_folder.join("versions.json");
    google_drive::download_versions_json(&versions_path)?;

    let folders = google_drive::list_files_in_folder("")?;
    let total_folders = folders.len();
    let mut processed_count = 0;

    for folder in folders {
        if folder.mime_type != "application/vnd.google-apps.folder" {
            continue;
        }

        if let Some(file) = google_drive::find_file_in_folder(&folder.id, "pack.zip")? {
            let result = modpack_processor::process_modpack(&file, local_folder, &folder.name);
            if result.is_some() {
                processed_count += 1;
                let progress = (processed_count as f64 / total_folders as f64 * 100.0).round() as u32;

                window.emit(
                    "progress",
                    serde_json::json!({
                        "packName": folder.name,
                        "progress": progress
                    }),
                )?;
            }
        }
    }

    fs::remove_file(versions_path)?;
    Ok(())
}