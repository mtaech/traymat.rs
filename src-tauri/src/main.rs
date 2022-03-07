#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use serde::{Serialize, Deserialize};
fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler!(get_bing_list))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Deserialize,Serialize,Debug)]
struct BingInfo {
  images:Vec<ImageInfo>
}
#[derive(Deserialize,Serialize,Debug)]
struct ImageInfo{
  url:String,
  title:String,
  startdate:String
}

#[tauri::command]
fn get_bing_list() -> Result<Vec<ImageInfo>,String>{
  let bing_api = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=8&mkt=us_EN";
  let resp =  reqwest::blocking::get(bing_api).expect("get resp error".into() );
  let bing_info:BingInfo = resp.json().expect("Deserialize json error".into());
  Ok(bing_info.images)
}