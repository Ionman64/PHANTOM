extern crate project_analyser;
extern crate fern;
#[macro_use]
extern crate log;


use project_analyser::config;

use project_analyser::downloader;
use project_analyser::thread_helper::ThreadPool;
use project_analyser::downloader::{get_home_dir_path};
use project_analyser::git_analyser;
use std::path::Path;
use std::fs;

fn main() {
    match project_analyser::setup_logger() {
        Ok(_) => {},
        Err(_) => { panic!("Cannot setup logger, Programme will terminate") }
    };

    let projects = match downloader::read_project_urls_from_file(String::from("projects.csv")) {
        Ok(project_struct) => {
            info!("Finished reading projects file;");
            match project_struct.skipped_lines {
                None => {
                    info!("No lines skipped");
                },
                Some(lines) => {
                    warn!("Lines Skipped:");
                    for line in lines {
                        warn!("{}", line);
                    }
                },
            }
            project_struct.response
        }
        Err(_) => {
            panic!("Could not read the project URLs");
        }
    };
    let csv_path = Path::new(&get_home_dir_path().unwrap())
        .join("project_analyser")
        .join("analysis");
    fs::create_dir_all(&csv_path).expect("Could not create directories");

    let thread_pool = ThreadPool::new(75);
    for project in projects.into_iter() {
        thread_pool.execute(move || {
            println!("Spawned new thread!");
            let cloned_project = match downloader::clone_project(project) {
                Ok(cloned_project) => cloned_project,
                Err(_) => {
                    error!("Could not clone project");
                    return;
                },
            };
            match git_analyser::generate_git_log(&cloned_project) {
                Ok(log) => {
                    info!("Created log in {}", &cloned_project.path);
                    log
                }
                Err(e) => {
                    error!("Could not generate log file for project {}: {:?}", &cloned_project.path, e);
                    return
                }
            };
            let date_count = match git_analyser::count_commits_per_day(&cloned_project) {
                Ok(date_count) => { date_count },
                Err(_) => {
                    error!("Could not count commits");
                    return
                },
            };
            match git_analyser::generate_analysis_csv(&cloned_project, date_count) {
                Ok(_) => {},
                Err(_) => { error!("Could not generate analysis CSV") },
            }
        });
    }
}
