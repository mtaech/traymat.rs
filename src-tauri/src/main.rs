#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

extern crate core;

mod wallpaper;
mod env;

use log::{debug, error, info, trace, warn};

fn main() {
    env::setup_logger();
    tauri::Builder::default()
        .invoke_handler(
          tauri::generate_handler![
          wallpaper::set_wallpaper,
          wallpaper::get_bing_list
          ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_logger() -> Result<(), fern::InitError> {
  fern::Dispatch::new()
      .format(|out, message, record| {
        out.finish(format_args!(
          "{}[{}][{}] {}",
          chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
          record.target(),
          record.level(),
          message
        ))
      })
      .level(log::LevelFilter::Debug)
      .chain(std::io::stdout())
      .chain(fern::log_file("output.log")?)
      .apply()?;
  Ok(())
}