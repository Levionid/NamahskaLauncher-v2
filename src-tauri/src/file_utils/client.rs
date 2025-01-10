use reqwest::blocking::Client;
use std::fs::{self, create_dir_all, File};
use std::io::{self, copy};
use std::path::Path;

pub fn create_folder(path: &Path) -> io::Result<()> {
    if !path.exists() {
        create_dir_all(path)?;
    }
    Ok(())
}

pub fn move_overrides_to_root(modpack_folder: &Path) -> io::Result<()> {
    let overrides_path = modpack_folder.join("overrides");
    if overrides_path.exists() {
        for entry in fs::read_dir(overrides_path.clone())? {
            let entry = entry?;
            let dest = modpack_folder.join(entry.file_name());
            fs::rename(entry.path(), dest)?;
        }
        fs::remove_dir_all(overrides_path)?;
    }
    Ok(())
}

pub fn download_mod(url: &str, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = Client::new().get(url).send()?;
    if response.status().is_success() {
        let content = response.bytes()?;
        let mut content = content.as_ref();
        let mut dest = File::create(output_path)?;
        copy(&mut content, &mut dest)?;
        println!("Скачан мод: {}", output_path.display());
        Ok(())
    } else {
        Err(format!(
            "Ошибка загрузки с URL {}: статус {}",
            url,
            response.status()
        ))?
    }
}