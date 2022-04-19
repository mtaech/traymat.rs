#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod wallpaper;

fn main() {
    tauri::Builder::default()
        .invoke_handler(
          tauri::generate_handler![
          wallpaper::set_wallpaper,
          wallpaper::get_bing_list
          ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

