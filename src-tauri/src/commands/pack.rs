use crate::commands::utils::read_modrinth_file;
use crate::commands::packs::PATH;
use std::path::Path;
use serde::{Serialize, Deserialize};
use std::process::Command;


#[derive(Debug, Serialize, Deserialize)]
pub struct Pack {
    pub name: String,
    pub fabric: String,
    pub minecraft: String,
    pub play_time: String,
    pub last_played: String,
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

#[tauri::command]
pub async fn launch_pack(pack_name: &str, user_name: &str, minecraft_version: &str, fabric_version: &str) -> Result<String, String> {
    let python_path = r"programs\python\pythonw.exe";
    let script_path = r"programs\main.py";

    if !Path::new(python_path).exists() {
        return Err(format!("Python executable not found at {}", python_path));
    }
    if !Path::new(script_path).exists() {
        return Err(format!("Script file not found at {}", script_path));
    }

    let output = Command::new(python_path)
        .arg(script_path)
        .arg(pack_name)
        .arg(user_name)
        .arg(minecraft_version)
        .arg(fabric_version)
        .output()
        .map_err(|e| format!("Failed to execute process: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!("Ошибка выполнения: {}", String::from_utf8_lossy(&output.stderr)))
    }
}