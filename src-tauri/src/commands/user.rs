use serde_json::json;
use std::path::PathBuf;

#[tauri::command]
pub async fn get_nickname() -> Result<String, String> {
    let file = PathBuf::from("user.json");

    if file.exists() {
        let info_json = crate::commands::utils::read_modrinth_file(file.clone())?;
        Ok(info_json["nickname"].as_str().unwrap_or("username").to_string())
    } else {
        let nickname = "username".to_string();
        let info_data = json!({ "nickname": nickname });

        std::fs::write(
            &file,
            serde_json::to_string_pretty(&info_data).map_err(|e| format!("Ошибка сериализации JSON: {}", e))?,
        )
        .map_err(|e| format!("Ошибка записи файла: {}", e))?;

        Ok(nickname)
    }
}

#[tauri::command]
pub async fn set_nickname(nickname: String) -> Result<(), String> {
    let file = PathBuf::from("user.json");
    let info_data = json!({ "nickname": nickname });

    std::fs::write(
        &file,
        serde_json::to_string_pretty(&info_data).map_err(|e| format!("Ошибка сериализации JSON: {}", e))?,
    )
    .map_err(|e| format!("Ошибка записи файла: {}", e))?;

    Ok(())
}