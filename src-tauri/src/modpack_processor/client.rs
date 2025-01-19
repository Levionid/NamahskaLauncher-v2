use rayon::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicUsize, Arc};
use tauri::Emitter;
use tauri::Window;
use thiserror::Error;
use tokio::fs;
use zip::read::ZipArchive;
use zip::result::ZipError;
use std::{thread, time};

use crate::file_utils::{create_folder, download_mod, move_overrides_to_root};
use crate::google_drive::{download_file, FileInfo};

#[derive(Debug, Error)]
pub enum ModpackError {
    #[error("File I/O error: {0}")]
    FileIO(#[from] std::io::Error),
    #[error("JSON parsing error: {0}")]
    JsonParsing(#[from] serde_json::Error),
    #[error("ZIP error: {0}")]
    Zip(#[from] ZipError),
    #[error("Invalid modpack structure")]
    InvalidStructure,
    #[error("General error: {0}")]
    GeneralError(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Failed to process modpack: {0}")]
    ProcessingError(String),
    #[error("Error: {0}")]
    String(String)
}

impl From<reqwest::Error> for ModpackError {
    fn from(error: reqwest::Error) -> Self {
        ModpackError::GeneralError(Box::new(error))
    }
}

pub async fn process_modpack(
    file: FileInfo,
    local_folder: PathBuf,
    window: &Window,
) -> Result<PathBuf, ModpackError> {
    let folder_name = file.name.trim_end_matches(".mrpack");
    let local_modpack_path = local_folder.join(folder_name);
    let local_zip_path = local_folder.join(&file.name);

    let local_version = read_local_version(&local_modpack_path);
    let remote_version = read_remote_versions()?.get(folder_name).cloned();

    if let Some(remote_version) = remote_version {
        if local_version.as_deref() == Some(&remote_version) {
            println!(
                "Модпак {} уже актуален (версия {}).",
                folder_name, remote_version
            );
            return Ok(local_modpack_path);
        } else {
            println!(
                "Обновление модпака {} до версии {}...",
                folder_name, remote_version
            );
        }
    }

    clean_modpack_directory(&local_modpack_path)?;
    println!("Скачиваем модпак: {}", folder_name);

    download_file(&file.id, &file.name, local_folder.clone(), window)
        .await
        .map_err(|_| {
            ModpackError::ProcessingError(format!("Ошибка загрузки модпака {}", folder_name))
        })?;
    println!("Скачан: {}", local_modpack_path.display());

    extract_zip(&local_zip_path, &local_modpack_path)
        .map_err(|_| {
            ModpackError::ProcessingError(format!("Ошибка распаковки модпака {}", folder_name))
        })?;
    println!("Распакован: {}", local_modpack_path.display());

    fs::remove_file(&local_zip_path).await?;
    println!("Удален: {}", local_zip_path.display());

    move_overrides_to_root(local_modpack_path.clone())
        .await
        .map_err(|e| ModpackError::FileIO(e))?;

    let modrinth_index_path = local_modpack_path.join("modrinth.index.json");
    if modrinth_index_path.exists() {
        process_modrinth_file(modrinth_index_path, local_modpack_path.clone(), window)?;
    }

    Ok(local_modpack_path)
}

fn clean_modpack_directory(modpack_path: &Path) -> Result<(), ModpackError> {
    for folder in ["mods", "config"] {
        let path = modpack_path.join(folder);
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
            println!("Удалена папка: {}", path.display());
        }
    }

    let path = modpack_path.join("modrinth.index.json");
    if path.exists() {
        std::fs::remove_file(&path)?;
        println!("Удален файл: {}", path.display());
    }
    Ok(())
}

fn read_local_version(modpack_path: &Path) -> Option<String> {
    let path = modpack_path.join("modrinth.index.json");
    let data = std::fs::read_to_string(&path).ok()?;
    let json: Value = serde_json::from_str(&data).ok()?;
    json["versionId"].as_str().map(String::from)
}

fn read_remote_versions() -> Result<HashMap<String, String>, ModpackError> {
    let path = Path::new("modpacks/versions.json");
    let data = std::fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&data)?;
    let versions = json
        .as_object()
        .ok_or(ModpackError::InvalidStructure)?
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
        .collect();
    Ok(versions)
}

fn extract_zip(zip_path: &Path, extract_to: &Path) -> Result<(), ModpackError> {
    let file = std::fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = extract_to.join(file.mangled_name());

        if file.is_dir() {
            create_folder(outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                create_folder(parent.to_path_buf())?;
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

fn process_modrinth_file(
    index_path: PathBuf,
    modpack_folder: PathBuf,
    window: &Window,
) -> Result<(), ModpackError> {
    let data = std::fs::read_to_string(index_path)?;
    let modrinth_data: Value = serde_json::from_str(&data)?;
    let files = modrinth_data["files"]
        .as_array()
        .ok_or(ModpackError::InvalidStructure)?;

    let total_files = files.len();
    let processed_count = Arc::new(AtomicUsize::new(0));
    let delay_duration = time::Duration::from_millis(100);

    files.par_iter().try_for_each(|file| {
        let download_url = file["downloads"][0]
            .as_str()
            .ok_or(ModpackError::InvalidStructure)?;
        let file_path = modpack_folder.join(file["path"].as_str().unwrap());
        create_folder(file_path.parent().unwrap().to_path_buf())?;

        println!("Скачиваем мод: {}", download_url);
        download_mod(download_url, file_path).map_err(|err| {
            ModpackError::ProcessingError(format!(
                "Ошибка загрузки модпака {}: {}",
                modpack_folder.display(),
                err
            ))
        })?;

        thread::sleep(delay_duration);

        let current_count = processed_count.fetch_add(1, Ordering::SeqCst) + 1;
        let progress = (current_count as f64 / total_files as f64 * 100.0).round() as u32;

        window
            .emit(
                "progress",
                serde_json::json!({
                    "progress": progress,
                    "packName": file["path"].as_str().unwrap_or("Unknown")
                }),
            )
            .map_err(|e| ModpackError::ProcessingError(e.to_string()))?;
        Ok::<(), ModpackError>(())
    })?;

    Ok(())
}