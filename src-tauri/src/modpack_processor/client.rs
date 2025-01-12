use std::fs;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::collections::HashMap;
use std::error::Error;
use std::io::Error as IoError;

use zip::read::ZipArchive;
use rayon::prelude::*;

use crate::file_utils::{create_folder, move_overrides_to_root};
use crate::google_drive::{download_file, FileInfo};
use crate::file_utils;

pub fn process_modpack(file: &FileInfo, local_folder: &Path, folder_name: &str) -> Option<PathBuf> {
    let local_modpack_path = local_folder.join(folder_name);
    let local_zip_path = local_folder.join(&file.name);
    
    let local_version = read_local_version(&local_modpack_path);
    let remote_version = read_remote_versions().ok()?.get(folder_name)?.clone();


    if local_version.as_deref() == Some(&remote_version) {
        println!("Модпак {} уже актуален (версия {}).", folder_name, remote_version);
        return Some(local_modpack_path);
    }
    else if !local_version.is_none() {
        println!("Обновление модпака {} до версии {}...", folder_name, remote_version);
    }

    if let Err(e) = clean_modpack_directory(&local_modpack_path) {
        eprintln!("Ошибка очистки папки модпака: {}", e);
        return None;
    }

    if download_file(&file.id, &file.name, &local_zip_path).is_err()
        || extract_zip(&local_zip_path, &local_modpack_path).is_err()
    {
        eprintln!("Ошибка обработки модпака {}", folder_name);
        return None;
    }

    fs::remove_file(&local_zip_path).ok();
    move_overrides_to_root(&local_modpack_path).ok();

    let modrinth_index_path = local_modpack_path.join("modrinth.index.json");
    if modrinth_index_path.exists() {
        process_modrinth_file(&modrinth_index_path, &local_modpack_path).ok();
    }

    Some(local_modpack_path)
}

fn clean_modpack_directory(modpack_path: &Path) -> Result<(), Box<dyn Error>> {
    for folder in ["mods", "config"] {
        let path = modpack_path.join(folder);
        if path.exists() {
            fs::remove_dir_all(&path)?;
            println!("Удалена папка: {}", path.display());
        }
    }
    Ok(())
}

fn read_local_version(modpack_path: &Path) -> Option<String> {
    let path = modpack_path.join("modrinth.index.json");
    if !path.exists() {
        return None;
    }
    let data = fs::read_to_string(path).ok()?;
    let json: Value = serde_json::from_str(&data).ok()?;
    json["versionId"].as_str().map(String::from)
}

fn read_remote_versions() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let path = Path::new("modpacks/versions.json");
    let data = fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&data)?;
    let versions = json
        .as_object()
        .ok_or("Некорректная структура versions.json")?
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
        .collect();
    Ok(versions)
}

fn extract_zip(zip_path: &Path, extract_to: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = extract_to.join(file.mangled_name());

        if file.is_dir() {
            create_folder(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                create_folder(parent)?;
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

fn process_modrinth_file(
    index_path: &Path,
    modpack_folder: &Path,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let data = std::fs::read_to_string(index_path)?;
    let modrinth_data: serde_json::Value = serde_json::from_str(&data)?;
    let files = modrinth_data["files"].as_array().unwrap();

    let modpack_folder = Arc::new(modpack_folder.to_path_buf());

    files.par_iter().try_for_each(|file| {
        let download_url = file["downloads"][0]
            .as_str()
            .ok_or_else(|| Box::new(IoError::new(std::io::ErrorKind::InvalidData, "Invalid URL")) as Box<dyn Error + Send + Sync>)?;
        let file_path = modpack_folder.join(file["path"].as_str().unwrap());
        std::fs::create_dir_all(file_path.parent().unwrap())?;
        println!("Скачиваем мод: {}", download_url);
        if let Err(e) = file_utils::download_mod(download_url, &file_path) {
            eprintln!("Ошибка скачивания мода {}: {}", download_url, e);
        }
        // Adding a small delay to prevent overloading the processor
        std::thread::sleep(std::time::Duration::from_millis(50));
        Ok::<(), Box<dyn Error + Send + Sync>>(())
    })?;

    Ok(())
}