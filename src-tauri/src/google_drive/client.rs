use reqwest::Client;
use serde::Deserialize;
use std::path::PathBuf;
use thiserror::Error;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

/// Константы для Google API
const API_KEY: &str = "AIzaSyANoX7JsJH-hXe9r7q7DccImDvD_zINc5Y";
const FOLDER_ID: &str = "1cAaVaq0ufSKYQMKLEYjnoO4nTLkTozOG";

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),
    #[error("File I/O error: {0}")]
    FileIO(#[from] std::io::Error),
    #[error("JSON parsing error: {0}")]
    JsonParsing(#[from] serde_json::Error),
    #[error("Unexpected response format")]
    UnexpectedResponse,
}

#[derive(Deserialize, Clone)]
pub struct FileInfo {
    pub id: String,
    pub name: String,
    pub mime_type: String,
}

pub async fn create_folder(path: PathBuf) -> Result<(), ClientError> {
    if !path.exists() {
        fs::create_dir_all(path).await?;
    }
    Ok(())
}

pub async fn list_files_in_folder() -> Result<Vec<FileInfo>, ClientError> {
    let url = format!(
        "https://www.googleapis.com/drive/v3/files?q='{}'+in+parents&key={}",
        FOLDER_ID, API_KEY
    );

    let client = Client::new();
    let response = client.get(&url).send().await?;
    let json: serde_json::Value = response.json().await?;

    let files = json["files"]
        .as_array()
        .ok_or(ClientError::UnexpectedResponse)?
        .iter()
        .filter_map(|item| {
            Some(FileInfo {
                id: item["id"].as_str()?.to_string(),
                name: item["name"].as_str()?.to_string(),
                mime_type: item["mimeType"].as_str()?.to_string(),
            })
        })
        .collect();

    Ok(files)
}

pub async fn download_file(
    file_id: &str,
    file_name: &str,
    output_path: PathBuf,
) -> Result<(), ClientError> {
    let url = format!(
        "https://www.googleapis.com/drive/v3/files/{}?alt=media&key={}",
        file_id, API_KEY
    );

    let client = Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let mut dest = File::create(output_path.join(file_name)).await?;
        let content = response.bytes().await?;
        dest.write_all(&content).await?;
        println!("Downloaded file: {}", file_name);
        Ok(())
    } else {
        Err(ClientError::HttpRequest(
            response.error_for_status().err().unwrap(),
        ))
    }
}

pub async fn move_overrides_to_root(modpack_folder: PathBuf) -> Result<(), ClientError> {
    let overrides_path = modpack_folder.join("overrides");
    if overrides_path.exists() {
        let mut dir = fs::read_dir(&overrides_path).await?;
        while let Some(entry) = dir.next_entry().await? {
            let dest = modpack_folder.join(entry.file_name());
            fs::rename(entry.path(), dest).await?;
        }
        fs::remove_dir_all(overrides_path).await?;
    }
    Ok(())
}

pub async fn download_versions_json(output_path: PathBuf) -> Result<(), ClientError> {
    const VERSIONS_FILE_ID: &str = "11GWqfPHggIgonQYB2yUEVCiUu-fkUqXJ";
    let file_name = "versions.json";
    println!("Downloading versions.json...");
    download_file(VERSIONS_FILE_ID, file_name, output_path).await
}