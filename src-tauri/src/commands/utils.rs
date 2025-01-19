use serde_json::Value;
use std::path::PathBuf;
use tauri::Emitter;

pub fn read_modrinth_file(path: PathBuf) -> Result<Value, String> {
    let contents = std::fs::read_to_string(&path)
        .map_err(|e| format!("Ошибка при чтении файла {}: {}", path.display(), e))?;

    serde_json::from_str(&contents).map_err(|e| format!("Ошибка при парсинге файла {}: {}", path.display(), e))
}

pub fn get_exe_dir() -> Result<PathBuf, String> {
    std::env::current_exe()
        .map_err(|e| format!("Ошибка при получении пути к исполняемому файлу: {}", e))
        .and_then(|exe_path| {
            exe_path
                .parent()
                .map(|dir| dir.to_path_buf())
                .ok_or_else(|| "Не удалось получить директорию исполняемого файла".to_string())
        })
}

pub fn window_emit(window: &tauri::Window, process: u64, message: &str) -> Result<(), crate::modpack_processor::ModpackError> {
    window
        .emit(
            "progress",
            serde_json::json!({ "progress": process, "message": message }),
        )
        .map_err(|e| crate::modpack_processor::ModpackError::ProcessingError(e.to_string()))
}