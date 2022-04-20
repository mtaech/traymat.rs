use std::fs;
use std::path::Path;
use log::{debug, error, info, trace, warn};


pub fn setup_logger() -> Result<(), fern::InitError> {
    let log_path = get_log_path();
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
        .chain(fern::log_file(log_path)?)
        .apply()?;
    Ok(())
}
 
pub fn get_log_path() -> String{
    let home = env!("HOME");
    let wave_path = Path::new(home).join(".wave");
    let path = wave_path.join("wave.log").into_os_string().into_string().unwrap();
    if !wave_path.exists() {
        fs::create_dir_all(wave_path).unwrap();
    }
    path
}