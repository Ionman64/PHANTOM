extern crate fern;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate chrono;

pub mod models;
pub mod thread_helper;
pub mod config;
pub mod extract_measures_and_features;

use std::env;
use std::path::PathBuf;
use std::io::ErrorKind;

pub const ROOT_FOLDER:&str = "project_downloader";

pub fn setup_logger() -> Result<(), fern::InitError> {
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
        .level(log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

pub fn get_home_dir_path() -> Result<String, ErrorKind> {
    let home_dir = match env::home_dir() {
        None => PathBuf::from(""),
        Some(path) => PathBuf::from(path),
    };
    match home_dir.into_os_string().into_string() {
        Ok(s) => Ok(s),
        Err(_) => {
            error!("Could not convert home dir into string.");
            return Err(ErrorKind::Other); //("Could not convert home dir into string");
        }
    }
}

pub fn get_root_folder() -> String {
    return String::from(ROOT_FOLDER)
}

