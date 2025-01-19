use crate::{file_utils, google_drive, modpack_processor};
use crate::modpack_processor::ModpackError;
use std::path::{PathBuf, Path};
use tauri::{path, Window};
use serde::{Serialize, Deserialize};
use std::process::Command;
use tauri::Emitter;
use base64::encode;

const PATH: &str = "../modpacks";

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
    Ok(encode(&image_data))
}

fn window_emit(window: &Window, process: u64, pack_name: &str) -> Result<(), ModpackError>{
    window
    .emit(
        "progress",
        serde_json::json!({
            "progress": process,
            "packName": pack_name
        }),
    )
    .map_err(|e| ModpackError::ProcessingError(e.to_string()))?;
    Ok::<(), ModpackError>(())
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Pack {
    name: String,
    fabric: String,
    minecraft: String,
    play_time: String,
    last_played: String,
}

#[tauri::command]
pub async fn get_nickname() -> Result<String, String> {
    let file = PathBuf::from("user.json");

    if file.exists() {
        let info_json = match read_modrinth_file(file.clone()) {
            Ok(j) => j,
            Err(e) => return Err(e),
        };

        let nickname = info_json["nickname"]
            .as_str()
            .unwrap_or("username")
            .to_string();

        Ok(nickname)
    } else {
        let nickname = "username".to_string();
        let info_data = serde_json::json!({
            "nickname": nickname,
        });

        if let Err(e) = std::fs::write(
            &file,
            serde_json::to_string_pretty(&info_data)
                .map_err(|e| format!("Ошибка сериализации JSON: {}", e))?,
        ) {
            return Err(format!("Ошибка записи файла: {}", e));
        }

        Ok(nickname)
    }
}

#[tauri::command]
pub async fn set_nickname(nickname: String) -> Result<(), String> {
    let file = std::path::Path::new("user.json");
    let info_data = serde_json::json!({
        "nickname": nickname,
    });

    std::fs::write(
        &file,
        serde_json::to_string_pretty(&info_data)
            .map_err(|e| format!("Ошибка сериализации JSON: {}", e))?,
    )
    .map_err(|e| format!("Ошибка записи файла: {}", e))?;

    Ok(())
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
                    let info_file_path = entry.path().join("info.json");
                    let mut play_time_json = "".to_string();
                    let mut last_played_json = "".to_string();

                    if info_file_path.exists() {
                        let info_json = match read_modrinth_file(info_file_path.clone()) {
                            Ok(j) => j,
                            Err(e) => return Err(e),
                        };

                        play_time_json = info_json["play-time"]
                            .as_str()
                            .unwrap_or("")
                            .to_string();
                        last_played_json = info_json["last-play"]
                            .as_str()
                            .unwrap_or("")
                            .to_string();
                    } else {
                        play_time_json = "0".to_string();
                        last_played_json = "0".to_string();

                        // Создание файла info.json
                        let info_data = serde_json::json!({
                            "play-time": play_time_json,
                            "last-play": last_played_json,
                        });

                        if let Err(e) = std::fs::write(
                            &info_file_path,
                            serde_json::to_string_pretty(&info_data)
                                .map_err(|e| format!("Ошибка сериализации JSON: {}", e))?,
                        ) {
                            return Err(format!(
                                "Ошибка при создании info.json в {}: {}",
                                info_file_path.display(),
                                e
                            ));
                        }
                    }

                    if modrinth_file_path.exists() {
                        let modrinth_json = match read_modrinth_file(modrinth_file_path.clone()) {
                            Ok(j) => j,
                            Err(e) => return Err(e),
                        };

                        let fabric_version = modrinth_json["dependencies"]["fabric-loader"]
                            .as_str()
                            .unwrap_or("")
                            .to_string();
                        let minecraft_version = modrinth_json["dependencies"]["minecraft"]
                            .as_str()
                            .unwrap_or("")
                            .to_string();

                        packs.push(Pack {
                            name: name.to_string(),
                            fabric: fabric_version,
                            minecraft: minecraft_version,
                            play_time: play_time_json.clone(),
                            last_played: last_played_json.clone(),
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

#[tauri::command]
pub async fn launch_pack(pack_name: &str, user_name: &str, minecraft_version: &str, fabric_version: &str) -> Result<String, String> {
    let python_path = r"programs\python\pythonw.exe";
    let script_path = r"programs\main.py";

    // Проверьте существование файлов
    if !std::path::Path::new(python_path).exists() {
        return Err(format!("Python executable not found at {}", python_path));
    }
    if !std::path::Path::new(script_path).exists() {
        return Err(format!("Script file not found at {}", script_path));
    }

    // Запуск команды
    let output = Command::new(python_path)
        .arg(script_path)
        .arg(pack_name)
        .arg(user_name)
        .arg(minecraft_version)
        .arg(fabric_version)
        .output()
        .map_err(|e| format!("Failed to execute process: {}", e))?;

    // Проверка статуса выполнения команды
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!(
            "Ошибка выполнения: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

#[tauri::command]
pub async fn open_folder(path: String) -> Result<(), String> {
    let exe_dir = get_exe_dir()?;
    let path = exe_dir.join(path);
    println!("exe_dir/path open_folder {}", path.display());
    let output = Command::new("explorer")
        .arg(path)
        .output()
        .map_err(|e| format!("Ошибка при открытии папки: {}", e))?;

    if !output.status.success() {
        return Err(format!("Не удалось открыть папку: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

#[tauri::command]
pub async fn get_file_count(path: String) -> Result<u64, String> {
    let exe_dir = get_exe_dir()?;
    let path = exe_dir.join(path);
    println!("exe_dir/path get_file_count {}", path.display());
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

fn get_exe_dir() -> Result<PathBuf, String> {
    std::env::current_exe()
        .map_err(|e| format!("Ошибка при получении пути к исполняемому файлу: {}", e))
        .and_then(|exe_path| {
            exe_path
                .parent()
                .map(|dir| dir.to_path_buf())
                .ok_or_else(|| "Не удалось получить директорию исполняемого файла".to_string())
        })
}