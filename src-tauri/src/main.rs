#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::Path;
use serde::{Serialize, Deserialize};
fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler!(get_bing_list,set_wallpaper))
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

#[derive(Deserialize,Serialize,Debug)]
struct ResultApi {
  code:String,
  msg:String,
  data:String
}

impl ResultApi {
  fn new(code:String,msg:String,data:String)-> ResultApi{
    ResultApi{
      code,
      msg,
      data
    }
  }
}

#[tauri::command]
fn get_bing_list() -> Result<Vec<ImageInfo>,String>{
  let bing_api = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=8&mkt=us_EN";
  let resp =  reqwest::blocking::get(bing_api).expect("get resp error".into() );
  let bing_info:BingInfo = resp.json().expect("Deserialize json error".into());
  Ok(bing_info.images)
}

#[tauri::command]
fn set_wallpaper(url:&str,title:&str,date:&str) -> Result<ResultApi,String>{
  let rst = match download_image(url, title, date) {
    Ok(path) => {
      wallpaper::set_from_path(&*path);
      ResultApi::new("200".to_string(),"设置成功！".to_string(),"success".to_string())
    }
    Err(_) => {
      ResultApi::new("500".to_string(),"设置失败！".to_string(),"error".to_string())
    }
  };
  Ok(rst)
}


fn download_image(url:&str,title:&str,date:&str) -> Result<String,Box<dyn Error>> {
  let home_path = env::var("HOME")?;
  let path = Path::new(&home_path).join("Pictures").join(date.to_owned()+"-"+title+".jpg");
  let string = String::from(path.to_str().unwrap());
  let bing_domain = "https://www.bing.com".to_string();
  let new_url =  bing_domain.add(&*url.replace("1920x1080", "UHD"));
  let res = reqwest::blocking::get(new_url)?;
  let mut file = File::create(&path)?;
  let stream = res.bytes()?;
  file.write_all(&*stream).unwrap();
  Ok(string)
}