use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Deserialize, Serialize, Debug)]
pub struct BingInfo {
    pub images: Vec<ImageInfo>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ImageInfo {
    pub url: String,
    pub title: String,
    pub startdate: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResultApi {
    pub code: String,
    pub msg: String,
    pub data: String,
}

impl ResultApi {
    pub fn new(code: String, msg: String, data: String) -> ResultApi {
        ResultApi { code, msg, data }
    }
}

#[tauri::command]
pub async fn get_bing_list() -> Result<Vec<ImageInfo>, String> {
    let bing_api = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=8&mkt=us_EN";
    let resp = reqwest::get(bing_api).await;
    let bing_info: BingInfo = match resp {
        Ok(info) => match info.json::<BingInfo>().await {
            Ok(info) => info,
            Err(error) => {
                log::error!("get images info error reason:{:?}", error);
                panic!()
            }
        },
        Err(error) => {
            log::error!("get images info error reason:{:?}", error);
            panic!()
        }
    };
    info!("bing_list:{:?}", bing_info);
    Ok(bing_info.images)
}

#[tauri::command]
pub fn set_wallpaper(url: &str, title: &str, date: &str) -> Result<ResultApi, String> {
    let rst = match download_image(url, title, date) {
        Ok(path) => {
            wallpaper::set_from_path(&*path).expect("set paper failed");
            ResultApi::new(
                "200".to_string(),
                "设置成功！".to_string(),
                "success".to_string(),
            )
        }
        Err(error) => {
            error!("set wallpaer error {:#?}", error);
            ResultApi::new(
                "500".to_string(),
                "设置失败！".to_string(),
                "error".to_string(),
            )
        }
    };
    Ok(rst)
}

pub fn download_image(url: &str, title: &str, date: &str) -> Result<String, Box<dyn Error>> {
    match home::home_dir() {
        None => {
            panic!("not have home");
        }
        Some(home_path) => {
            let wallpaper_dir = home_path.join("Pictures").join("Wallpaper");
            info!("wallpaper_dir path:{:#?}", wallpaper_dir);

            if !wallpaper_dir.exists() {
                match fs::create_dir_all(&wallpaper_dir) {
                    Ok(_) => {
                        info!("dir create success")
                    }
                    Err(_) => {
                        panic!("create wallpaper dir error!")
                    }
                }
            }
            let path = wallpaper_dir.join(date.to_owned() + "-" + title + ".jpg");
            let string = String::from(path.to_str().unwrap());
            if !path.exists() {
                let bing_domain = "https://www.bing.com".to_string();
                let new_url = bing_domain.add(&*url.replace("1920x1080", "UHD"));
                let res = reqwest::blocking::get(new_url)?;
                let mut file = File::create(&path)?;
                let stream = res.bytes()?;
                file.write_all(&*stream).unwrap();
            }
            return Ok(string);
        }
    }
}
