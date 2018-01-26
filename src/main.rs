extern crate project_analyser;
extern crate fern;
#[macro_use]
extern crate log;

use project_analyser::downloader;
use project_analyser::thread_helper::ThreadPool;
use std::thread;
use project_analyser::downloader::{ClonedProject, GitHubProject};
use project_analyser::git_analyser;

fn main() {
    match project_analyser::setup_logger() {
        Ok(_) => {},
        Err(_) => { panic!("Cannot setup logger, Programme will terminate") }
    };

    let projects = match downloader::read_project_urls_from_file() {
        Ok(projectStruct) => {
            info!("Finished reading projects file;");
            match projectStruct.skipped_lines {
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
            projectStruct.projects
        }
        Err(_) => {
            panic!("Could not read the project URLs");
        }
    };

    let thread_pool = ThreadPool::new(75);
    for project in projects.into_iter() {
        thread_pool.execute(move || {
            println!("Spawned new thread!");
            let cloned_project = match downloader::clone_project(project) {
                Ok(cloned_project) => cloned_project,
                Err(e) => {
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
