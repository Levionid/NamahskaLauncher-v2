#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// Структура для хранения информации о сборке
#[derive(serde::Serialize, Clone)]
struct Pack {
  name: String,
  icon_url: String,
  minecraft_version: String,
  fabric_version: String,
  play_time: String,
  last_played: String,
  mods_count: u32,
  resource_packs_count: u32,
  worlds_count: u32,
  shaders_count: u32,
  mods_path: String,
  resource_packs_path: String,
  worlds_path: String,
  shaders_path: String,
  folder_path: String,
}

#[tauri::command]
// Функция для получения списка сборок
async fn get_packs() -> Result<Vec<Pack>, String> {
  // Замените на реальную логику получения данных из Google Drive
  Ok(vec![
    Pack {
      name: "NamashkaCraft Mix".to_string(),
      icon_url: "no-texture.png".to_string(),
      minecraft_version: "1.21.4".to_string(),
      fabric_version: "0.16.9".to_string(),
      play_time: "14.88 минут".to_string(),
      last_played: "01.01.1970".to_string(),
      mods_count: 1488,
      resource_packs_count: 52,
      worlds_count: 42,
      shaders_count: 0,
      mods_path: "path/to/mods".to_string(),
      resource_packs_path: "path/to/resourcepacks".to_string(),
      worlds_path: "path/to/worlds".to_string(),
      shaders_path: "path/to/shaders".to_string(),
      folder_path: "path/to/folder".to_string(),
    },
    Pack {
      name: "Adventure Pack".to_string(),
      icon_url: "adventure-icon.png".to_string(),
      minecraft_version: "1.18.2".to_string(),
      fabric_version: "0.12.12".to_string(),
      play_time: "5.5 часов".to_string(),
      last_played: "12.09.2023".to_string(),
      mods_count: 120,
      resource_packs_count: 10,
      worlds_count: 3,
      shaders_count: 2,
      mods_path: "path/to/adventure/mods".to_string(),
      resource_packs_path: "path/to/adventure/resourcepacks".to_string(),
      worlds_path: "path/to/adventure/worlds".to_string(),
      shaders_path: "path/to/adventure/shaders".to_string(),
      folder_path: "path/to/adventure/folder".to_string(),
    },
    Pack {
      name: "Survival Pack".to_string(),
      icon_url: "survival-icon.png".to_string(),
      minecraft_version: "1.19.1".to_string(),
      fabric_version: "0.14.8".to_string(),
      play_time: "20.3 часов".to_string(),
      last_played: "05.10.2023".to_string(),
      mods_count: 200,
      resource_packs_count: 15,
      worlds_count: 5,
      shaders_count: 1,
      mods_path: "path/to/survival/mods".to_string(),
      resource_packs_path: "path/to/survival/resourcepacks".to_string(),
      worlds_path: "path/to/survival/worlds".to_string(),
      shaders_path: "path/to/survival/shaders".to_string(),
      folder_path: "path/to/survival/folder".to_string(),
    },
  ])
}

#[tauri::command]
// Функция для запуска сборки
async fn launch_pack(pack_name: String) -> Result<(), String> {
  println!("Запуск сборки: {}", pack_name);
  // Замените на реальную логику запуска Minecraft с выбранной сборкой
  Ok(())
}

#[tauri::command]
// Функция для открытия папки в проводнике
fn open_folder(path: String) -> Result<(), String> {
  println!("Открытие папки: {}", path);
  // Замените на реальную логику открытия папки
  Ok(())
}

#[tauri::command]
// Функция для открытия окна настроек
fn open_settings(_window: tauri::Window) -> Result<(), String> {
  // Создайте окно настроек
  // ...

  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      get_packs,
      launch_pack,
      open_folder,
      open_settings
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}