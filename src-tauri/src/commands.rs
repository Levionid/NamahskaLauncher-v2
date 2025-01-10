use crate::google_drive;
use tokio::task;
use crate::file_utils;

#[derive(serde::Serialize, Clone, Debug)]
pub struct Pack {
    pub name: String,
    pub id: String,
}

#[tauri::command]
pub async fn get_packs() -> Result<Vec<Pack>, String> {
    let result = task::block_in_place(|| {
        let mut packs: Vec<Pack> = Vec::new();
        let folders = google_drive::list_files_in_folder("").map_err(|e| e.to_string())?;
        
        for folder in folders {
            if folder.mime_type != "application/vnd.google-apps.folder" {
                continue;
            }
            file_utils::create_folder(std::path::Path::new(&format!("modpacks/{}", folder.name))).ok();

            let pack = google_drive::find_file_in_folder(&folder.id, "pack.zip")
                .map_err(|e| e.to_string())?
                .ok_or("Pack not found")?;

            packs.push(Pack {
                name: folder.name,
                id: pack.id,
            });
        }

        Ok(packs)
    });

    match result {
        Ok(packs) => Ok(packs),
        Err(e) => Err(e),
    }
}