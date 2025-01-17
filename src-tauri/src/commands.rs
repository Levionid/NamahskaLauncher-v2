use crate::{file_utils, google_drive, modpack_processor};
use std::path::{PathBuf, Path};
use tauri::Window;
use serde::Serialize;

const PATH: &str = "../modpacks";

#[tauri::command]
pub async fn modpacks_load(window: Window) -> Result<String, String> {
    check_internet_connection().await?;

    let local_folder = PathBuf::from(PATH);
    file_utils::create_folder(local_folder.clone())
        .map_err(|e| format!("Failed to create local folder: {}", e))?;

    google_drive::download_versions_json(local_folder.clone())
        .await
        .map_err(|e| format!("Failed to download versions.json: {}", e))?;

    let files = google_drive::list_files_in_folder()
        .await
        .map_err(|e| format!("Failed to list files in Google Drive: {}", e))?;

    process_files(files, local_folder, &window).await?;

    Ok("Загружено!".into())
}

async fn check_internet_connection() -> Result<(), String> {
    if tokio::net::TcpStream::connect("8.8.8.8:53").await.is_err() {
        Err("No internet connection".to_string())
    } else {
        Ok(())
    }
}

async fn process_files(
    files: Vec<google_drive::FileInfo>,
    local_folder: PathBuf,
    window: &Window,
) -> Result<(), String> {
    for file in files {
        println!("Processing file: {}", file.name);

        if file.name.ends_with(".mrpack") {
            match modpack_processor::process_modpack(file.clone(), local_folder.clone(), window)
                .await
            {
                Ok(path) => println!("Processed modpack: {}", path.display()),
                Err(e) => eprintln!("Error processing modpack {}: {:?}", file.name, e),
            }
        } else {
            println!("Skipping unsupported file: {}", file.name);
        }
    }
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct Pack {
    name: String,
    fabric: String,
    minecraft: String,
}

#[tauri::command]
pub async fn get_packs() -> Result<Vec<Pack>, String> {
    let modpacks_path = Path::new(PATH);
    if !modpacks_path.exists() || !modpacks_path.is_dir() {
        return Err("Директория 'modpacks' не существует или не является папкой.".into());
    }

    let mut packs = Vec::new();
    let entries = match std::fs::read_dir(modpacks_path) {
        Ok(e) => e,
        Err(e) => return Err(format!("Ошибка при чтении директории: {}", e)),
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => return Err(format!("Ошибка при чтении элемента директории: {}", e)),
        };

        if let Ok(metadata) = entry.metadata() {
            if metadata.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    let modrinth_file_path = entry.path().join("modrinth.index.json");

                    if modrinth_file_path.exists() {
                        let json = match read_modrinth_file(modrinth_file_path.clone()) {
                            Ok(j) => j,
                            Err(e) => return Err(e),
                        };

                        let fabric_version = match json["dependencies"]["fabric-loader"].as_str() {
                            Some(v) => v.to_string(),
                            None => return Err(format!("'fabric-loader' не найден в {}", modrinth_file_path.display())),
                        };

                        let minecraft_version = match json["dependencies"]["minecraft"].as_str() {
                            Some(v) => v.to_string(),
                            None => return Err(format!("'minecraft' не найден в {}", modrinth_file_path.display())),
                        };

                        packs.push(Pack {
                            name: name.to_string(),
                            fabric: fabric_version,
                            minecraft: minecraft_version,
                        });
                    }
                }
            }
        }
    }

    Ok(packs)
}

fn read_modrinth_file(path: PathBuf) -> Result<serde_json::Value, String> {
    let contents = match std::fs::read_to_string(path.clone()) {
        Ok(c) => c,
        Err(e) => return Err(format!("Ошибка при чтении файла {}: {}", path.display(), e)),
    };

    match serde_json::from_str(&contents) {
        Ok(json) => Ok(json),
        Err(e) => return Err(format!("Ошибка при парсинге файла {}: {}", path.display(), e)),
    }
}