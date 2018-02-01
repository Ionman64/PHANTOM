extern crate project_analyser;
extern crate fern;
#[macro_use]
extern crate log;
extern crate chrono;

use project_analyser::models::*;
use project_analyser::database;
use project_analyser::downloader;
use project_analyser::thread_helper::ThreadPool;
use project_analyser::downloader::get_home_dir_path;
use project_analyser::git_analyser;
use std::path::Path;
use std::fs;
use std::io::ErrorKind;
use project_analyser::utils::{detect_all_peaks, PEAK};


fn main() {
    match project_analyser::setup_logger() {
        Ok(_) => {}
        Err(_) => { panic!("Cannot setup logger, Programme will terminate") }
    };

    {
        database::establish_connection(); // Test connection
    }

    execute(String::from("projects.csv"));
}

fn execute(path_to_projects_csv: String) {
    let new_repositories = match downloader::read_project_urls_from_file(path_to_projects_csv) {
        Ok(project_struct) => {
            match project_struct.skipped_lines {
                None => {
                    info!("Read projects from csv with success.");
                }
                Some(lines) => {
                    warn!("Read project from csv with succes, but skipped lines: {:?}", lines);
                }
            }
            project_struct.response
        }
        Err(_) => {
            panic!("Reading project from csv failed");
        }
    };

    let mut git_repositories: Vec<GitRepository> = Vec::new();
    for project in new_repositories.into_iter().skip(297) {
        let url = project.url.clone();
        match database::create_git_repository(project) {
            Ok(repository) => git_repositories.push(repository),
            Err(ErrorKind::AlreadyExists) => {
                let repository = database::read_git_repository(url).unwrap();
                git_repositories.push(repository);
            }
            Err(_) => panic!("Problem With Database"),
        } //TODO: REMOVE PANIC HERE
    }

    let csv_path = Path::new(&get_home_dir_path().unwrap())
        .join("project_analyser")
        .join("analysis");
    fs::create_dir_all(&csv_path).expect("Could not create directories");

    let thread_pool = ThreadPool::new(75);
    for project in git_repositories.into_iter() {
        thread_pool.execute(move || {
            println!("Spawned new thread!");
            let cloned_project = match downloader::clone_project(project) {
                Ok(cloned_project) => cloned_project,
                Err(_) => {
                    error!("Could not clone project");
                    return;
                }
            };
            match git_analyser::generate_git_log(&cloned_project) {
                Ok(log) => {
                    info!("Created log in {}", &cloned_project.path);
                    log
                }
                Err(e) => {
                    error!("Could not generate log file for project {}: {:?}", &cloned_project.path, e);
                    return;
                }
            };
            let date_count = match git_analyser::count_commits_per_day(&cloned_project) {
                Ok(date_count) => { date_count }
                Err(_) => {
                    error!("Could not count commits");
                    return;
                }
            };

            // write commit frequency analysis into database
            for (date, frequency) in date_count.into_iter() {
                let entry = CommitFrequency {
                    repository_id: cloned_project.github.id,
                    commit_date: date.and_hms(0, 0, 0),
                    frequency,
                };
                match database::create_commit_frequency(entry) {
                    Ok(_) => {}
                    Err(_) => {
                        error!("Could not create frequency");
                    }
                }
            }
        });
    }
}
