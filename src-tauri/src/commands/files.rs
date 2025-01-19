use std::process::Command;
use crate::commands::utils;
use base64::{Engine, engine::general_purpose};

#[tauri::command]
pub async fn open_folder(path: String) -> Result<(), String> {
    let exe_dir = utils::get_exe_dir()?;
    let path = exe_dir.join(path);

    Command::new("explorer")
        .arg(path)
        .output()
        .map_err(|e| format!("Ошибка при открытии папки: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_file_count(path: String) -> Result<u64, String> {
    let exe_dir = utils::get_exe_dir()?;
    let path = exe_dir.join(path);
    let mut count = 0;

    if path.exists() {
        let mut paths = tokio::fs::read_dir(path).await.map_err(|e| format!("Ошибка: {}", e))?;
        while let Some(entry) = paths.next_entry().await.unwrap() {
            if let Ok(file_type) = entry.file_type().await {
                if file_type.is_file() {
                    count += 1;
                }
            }
        }
    } else {
        eprintln!("Директория {} отсутствует", path.display());
    }

    Ok(count)
}

#[tauri::command]
pub fn get_icon_base64(pack_name: String) -> Result<String, String> {
    let exe_dir = std::env::current_exe().map_err(|e| e.to_string())?;
    let icon_path = exe_dir
        .parent()
        .ok_or("Failed to get exe directory")?
        .join("modpacks")
        .join(&pack_name)
        .join("icon.png");

    if !icon_path.exists() {
        return Err(format!("Icon not found at {:?}", icon_path));
    }

    let image_data = std::fs::read(&icon_path).map_err(|e| e.to_string())?;
    let encoded = general_purpose::STANDARD.encode(&image_data);
    Ok(encoded)
}