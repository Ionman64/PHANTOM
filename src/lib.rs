extern crate fern;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate chrono;

pub mod models;
pub mod thread_helper;
pub mod downloader;
pub mod git_analyser;
pub mod config;


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


