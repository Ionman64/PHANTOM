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


fn main() {
    match project_analyser::setup_logger() {
        Ok(_) => {}
        Err(_) => { panic!("Cannot setup logger, Programme will terminate") }
    };
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
    for project in new_repositories.into_iter().take(1) {
        let url = project.url.clone();
        match database::create_git_repository(project) {
            Ok(repository) => git_repositories.push(repository),
            Err(ErrorKind::AlreadyExists) => {
                let repository = database::read_git_repository(url).unwrap();
                git_repositories.push(repository);
            }
            Err(_) => panic!("Problem With Database"),
        }
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
                Ok(_) => {
                    info!("Created log in {}", &cloned_project.path);
                },
                Err(ErrorKind::InvalidData) => {error!("Invalid Data")},
                Err(ErrorKind::InvalidInput) => {error!("Invalid input when creating log")},
                Err(ErrorKind::Other) => {error!("Unknown error creating log")}
                Err(e) => {
                    error!("Could not generate log file for project {}: {:?}", &cloned_project.path, e);
                    return;
                }
            };
            let _get_all_commits = match database::read_repository_commit(cloned_project.github.id) {
                Ok(x) => x,
                Err(_) => {error!("could not read commits");return;}
            };
            //git_analyser::checkout_commit(&cloned_project, &String::from("f4047650f2b654bb9ef33f2408212915e410e835"));
        });
    }
}
