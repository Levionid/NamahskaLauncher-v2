use crate::modpack_processor::ModpackError;
use std::io;
use std::path::PathBuf;
use tokio::fs;

pub fn create_folder(path: PathBuf) -> io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

pub async fn move_overrides_to_root(modpack_folder: PathBuf) -> io::Result<()> {
    let overrides_path = modpack_folder.join("overrides");
    if overrides_path.exists() {
        let mut dir = fs::read_dir(overrides_path.clone()).await?;
        while let Some(entry) = dir.next_entry().await? {
            let dest = modpack_folder.join(entry.file_name());
            if dest.exists() {
                if dest.is_dir() {
                    fs::remove_dir_all(&dest).await?;
                } else {
                    fs::remove_file(&dest).await?;
                }
            }
            fs::rename(entry.path(), dest).await?;
        }
        fs::remove_dir_all(overrides_path.clone()).await?;
    }
    println!("Удален: {}", overrides_path.display());
    Ok(())
}

pub fn download_mod(url: &str, output_path: PathBuf) -> Result<(), ModpackError> {
    let response = reqwest::blocking::get(url)?;
    if response.status().is_success() {
        let mut file = std::fs::File::create(output_path)?;
        let content = response.bytes()?;
        std::io::copy(&mut content.as_ref(), &mut file)?;
        Ok(())
    } else {
        Err(ModpackError::ProcessingError(format!(
            "Failed to download from URL: {}",
            url
        )))
    }
}