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
      icon_url: "no-texture.png".to_string(),
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
      icon_url: "no-texture.png".to_string(),
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
    Pack {
      name: "Creative Pack".to_string(),
      icon_url: "no-texture.png".to_string(),
      minecraft_version: "1.20.1".to_string(),
      fabric_version: "0.15.2".to_string(),
      play_time: "10.2 часов".to_string(),
      last_played: "15.08.2023".to_string(),
      mods_count: 80,
      resource_packs_count: 5,
      worlds_count: 2,
      shaders_count: 3,
      mods_path: "path/to/creative/mods".to_string(),
      resource_packs_path: "path/to/creative/resourcepacks".to_string(),
      worlds_path: "path/to/creative/worlds".to_string(),
      shaders_path: "path/to/creative/shaders".to_string(),
      folder_path: "path/to/creative/folder".to_string(),
    },
    Pack {
      name: "Hardcore Pack".to_string(),
      icon_url: "no-texture.png".to_string(),
      minecraft_version: "1.17.1".to_string(),
      fabric_version: "0.11.6".to_string(),
      play_time: "30.5 часов".to_string(),
      last_played: "20.07.2023".to_string(),
      mods_count: 150,
      resource_packs_count: 20,
      worlds_count: 1,
      shaders_count: 4,
      mods_path: "path/to/hardcore/mods".to_string(),
      resource_packs_path: "path/to/hardcore/resourcepacks".to_string(),
      worlds_path: "path/to/hardcore/worlds".to_string(),
      shaders_path: "path/to/hardcore/shaders".to_string(),
      folder_path: "path/to/hardcore/folder".to_string(),
    },
    Pack {
      name: "PvP Pack".to_string(),
      icon_url: "no-texture.png".to_string(),
      minecraft_version: "1.16.5".to_string(),
      fabric_version: "0.10.8".to_string(),
      play_time: "8.7 часов".to_string(),
      last_played: "10.06.2023".to_string(),
      mods_count: 60,
      resource_packs_count: 8,
      worlds_count: 0,
      shaders_count: 1,
      mods_path: "path/to/pvp/mods".to_string(),
      resource_packs_path: "path/to/pvp/resourcepacks".to_string(),
      worlds_path: "path/to/pvp/worlds".to_string(),
      shaders_path: "path/to/pvp/shaders".to_string(),
      folder_path: "path/to/pvp/folder".to_string(),
    },
    Pack {
      name: "Exploration Pack".to_string(),
      icon_url: "no-texture.png".to_string(),
      minecraft_version: "1.15.2".to_string(),
      fabric_version: "0.9.3".to_string(),
      play_time: "12.4 часов".to_string(),
      last_played: "25.05.2023".to_string(),
      mods_count: 90,
      resource_packs_count: 12,
      worlds_count: 4,
      shaders_count: 2,
      mods_path: "path/to/exploration/mods".to_string(),
      resource_packs_path: "path/to/exploration/resourcepacks".to_string(),
      worlds_path: "path/to/exploration/worlds".to_string(),
      shaders_path: "path/to/exploration/shaders".to_string(),
      folder_path: "path/to/exploration/folder".to_string(),
    },
    Pack {
      name: "Tech Pack".to_string(),
      icon_url: "no-texture.png".to_string(),
      minecraft_version: "1.14.4".to_string(),
      fabric_version: "0.8.2".to_string(),
      play_time: "18.9 часов".to_string(),
      last_played: "30.04.2023".to_string(),
      mods_count: 110,
      resource_packs_count: 18,
      worlds_count: 6,
      shaders_count: 3,
      mods_path: "path/to/tech/mods".to_string(),
      resource_packs_path: "path/to/tech/resourcepacks".to_string(),
      worlds_path: "path/to/tech/worlds".to_string(),
      shaders_path: "path/to/tech/shaders".to_string(),
      folder_path: "path/to/tech/folder".to_string(),
    },
    Pack {
      name: "Magic Pack".to_string(),
      icon_url: "no-texture.png".to_string(),
      minecraft_version: "1.13.2".to_string(),
      fabric_version: "0.7.1".to_string(),
      play_time: "22.1 часов".to_string(),
      last_played: "15.03.2023".to_string(),
      mods_count: 130,
      resource_packs_count: 25,
      worlds_count: 7,
      shaders_count: 5,
      mods_path: "path/to/magic/mods".to_string(),
      resource_packs_path: "path/to/magic/resourcepacks".to_string(),
      worlds_path: "path/to/magic/worlds".to_string(),
      shaders_path: "path/to/magic/shaders".to_string(),
      folder_path: "path/to/magic/folder".to_string(),
    },
    Pack {
      name: "Skyblock Pack".to_string(),
      icon_url: "no-texture.png".to_string(),
      minecraft_version: "1.12.2".to_string(),
      fabric_version: "0.6.4".to_string(),
      play_time: "16.6 часов".to_string(),
      last_played: "10.02.2023".to_string(),
      mods_count: 70,
      resource_packs_count: 7,
      worlds_count: 1,
      shaders_count: 2,
      mods_path: "path/to/skyblock/mods".to_string(),
      resource_packs_path: "path/to/skyblock/resourcepacks".to_string(),
      worlds_path: "path/to/skyblock/worlds".to_string(),
      shaders_path: "path/to/skyblock/shaders".to_string(),
      folder_path: "path/to/skyblock/folder".to_string(),
    },
    Pack {
      name: "Vanilla Pack".to_string(),
      icon_url: "no-texture.png".to_string(),
      minecraft_version: "1.11.2".to_string(),
      fabric_version: "0.5.2".to_string(),
      play_time: "5.3 часов".to_string(),
      last_played: "05.01.2023".to_string(),
      mods_count: 10,
      resource_packs_count: 2,
      worlds_count: 1,
      shaders_count: 0,
      mods_path: "path/to/vanilla/mods".to_string(),
      resource_packs_path: "path/to/vanilla/resourcepacks".to_string(),
      worlds_path: "path/to/vanilla/worlds".to_string(),
      shaders_path: "path/to/vanilla/shaders".to_string(),
      folder_path: "path/to/vanilla/folder".to_string(),
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