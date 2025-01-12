use reqwest::blocking::Client;
use serde::Deserialize;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::copy;

const API_KEY: &str = "AIzaSyANoX7JsJH-hXe9r7q7DccImDvD_zINc5Y";
const FOLDER_ID: &str = "1cAaVaq0ufSKYQMKLEYjnoO4nTLkTozOG";

#[derive(Deserialize)]
pub struct FileInfo {
    pub id: String,
    pub name: String,
    pub mime_type: String
}

pub fn find_file_in_folder(folder_id: &str, file_name: &str) -> Result<Option<FileInfo>, Box<dyn std::error::Error>> {
    let files = list_files_in_folder(folder_id)?;
    Ok(files.into_iter().find(|file| file.name == file_name))
}

pub fn list_files_in_folder(folder_id: &str) -> Result<Vec<FileInfo>, Box<dyn std::error::Error>> {
    let folder_id = if folder_id.is_empty() { FOLDER_ID } else { folder_id };

    let url = format!(
        "https://www.googleapis.com/drive/v3/files?q='{}'+in+parents&key={}",
        folder_id, API_KEY
    );

    let client = Client::new();
    let response = client.get(&url).send()?;
    let json: serde_json::Value = response.json()?;

    let mut files = Vec::new();
    if let Some(items) = json["files"].as_array() {
        for item in items {
            files.push(FileInfo {
                id: item["id"].as_str().unwrap().to_string(),
                name: item["name"].as_str().unwrap().to_string(),
                mime_type: item["mimeType"].as_str().unwrap().to_string()
            });
        }
    }

    Ok(files)
}

pub fn download_file(
    file_id: &str,
    file_name: &str,
    output_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.googleapis.com/drive/v3/files/{}?alt=media&key={}",
        file_id, API_KEY
    );

    let client = Client::new();
    let response = client.get(&url).send()?;

    
    if response.status().is_success() {
        let mut dest = File::create(output_path.join(file_name))?;
        let content = response.bytes()?;
        copy(&mut content.as_ref(), &mut dest)?;
        println!("Скачан файл: {}", file_name);
    } else {
        eprintln!("Ошибка при скачивании файла {}: {:?}", file_name, response.status());
    }

    Ok(())
}

pub fn download_versions_json(output_path: &Path) -> Result<(), Box<dyn Error>> {
    let file_id = "1ULBax2TUMwJlxMZf0tgy9QHlkwSdUYXg";
    download_file(file_id, "versions.json", output_path)
}