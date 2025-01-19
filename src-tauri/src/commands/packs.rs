use crate::commands::utils::*;
use crate::file_utils;
use crate::google_drive;
use crate::modpack_processor;
use tauri::Window;
use std::path::PathBuf;

pub const PATH: &str = "modpacks";

#[tauri::command]
pub async fn modpacks_load(window: Window) -> Result<String, String> {
    // Проверяем подключение к интернету
    check_internet_connection().await.map_err(|e| e)?;

    // Создаём локальную папку
    window_emit(&window, 0, "Создание папки modpacks").map_err(|e| format!("Failed to emit window: {}", e))?;
    let local_folder = PathBuf::from(PATH);
    file_utils::create_folder(local_folder.clone())
        .map_err(|e| format!("Failed to create local folder: {}", e))?;

    // Скачиваем файл версий
    window_emit(&window, 30, "modrinth.index.json").map_err(|e| format!("Failed to emit window: {}", e))?;
    google_drive::download_versions_json(local_folder.clone(), &window)
        .await
        .map_err(|e| format!("Failed to download versions.json: {}", e))?;

    // Список модов
    window_emit(&window, 70, "Загрузка списка модов").map_err(|e| format!("Failed to emit window: {}", e))?;
    let files = google_drive::list_files_in_folder()
        .await
        .map_err(|e| format!("Failed to list files in Google Drive: {}", e))?;

    // Обработка файлов модов
    window_emit(&window, 100, "Загрузка списка модов").map_err(|e| format!("Failed to emit window: {}", e))?;
    process_files(files, local_folder.clone(), &window).await?;

    // Удаление файла versions.json
    std::fs::remove_file(local_folder.join("versions.json")).map_err(|e| format!("Failed to delete versions.json: {}", e))?;

    Ok("Загружено!".to_string())
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
        // Лог для отладки
        println!("Processing file: {}", file.name);

        // Уведомление о начале обработки
        window_emit(&window, 0, format!("Загрузка: {}", file.name).as_str())
            .map_err(|e| format!("Failed to emit window: {}", e))?;

        // Проверка расширения файла
        if file.name.ends_with(".mrpack") {
            // Асинхронная обработка модпака
            match modpack_processor::process_modpack(file.clone(), local_folder.clone(), window).await {
                Ok(path) => {
                    println!("Processed modpack: {}", path.display());
                    window_emit(&window, 100, format!("Загружен: {}", path.display()).as_str())
                        .map_err(|e| format!("Failed to emit window: {}", e))?;
                }
                Err(e) => {
                    eprintln!("Error processing modpack {}: {:?}", file.name, e);
                }
            }
        } else {
            // Если файл не поддерживается
            println!("Skipping unsupported file: {}", file.name);
        }
    }

    Ok(())
}