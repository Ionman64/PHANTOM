extern crate project_analyser;
extern crate fern;
#[macro_use]
extern crate log;
extern crate chrono;

use project_analyser::models::*;
use project_analyser::database;
use project_analyser::downloader;
use project_analyser::thread_helper::ThreadPool;
use project_analyser::git_analyser;
use std::io::ErrorKind;


fn main() {
    match project_analyser::setup_logger() {
        Ok(_) => {}
        Err(_) => { panic!("Logger setup failed") }
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
            panic!("Reading projects from csv failed");
        }
    };

    let mut git_repositories: Vec<GitRepository> = Vec::new();
    for project in new_repositories.into_iter() {
        let url = project.url.clone();
        match database::create_git_repository(project) {
            Ok(repository) => git_repositories.push(repository),
            Err(ErrorKind::AlreadyExists) => {
                let repository = database::read_git_repository(url).unwrap();
                git_repositories.push(repository);
            }
            Err(_) => panic!("Failed to read git repositories"),
        }
    }

    let thread_pool = ThreadPool::new(100);
    for project in git_repositories.into_iter().take(5) {
        thread_pool.execute(move || {
            let project_id = project.id.clone();
            let cloned_project = match downloader::clone_project(project) {
                Ok(cloned_project) => cloned_project,
                Err(_) => {
                    error!("Failed to clone project {}. Skipping project.", project_id);
                    return;
                }
            };
            match git_analyser::generate_git_log(&cloned_project) { // TODO rethink error messages
                Ok(_) => {
                    info!("Created log in {}", &cloned_project.path);
                }
                Err(ErrorKind::InvalidData) => { error!("Invalid Data") }
                Err(ErrorKind::InvalidInput) => { error!("Invalid input when creating log") }
                Err(ErrorKind::Other) => { error!("Unknown error creating log") }
                Err(e) => {
                    error!("Failed to generate git log for project {}: {:?}", &cloned_project.path, e);
                    return;
                }
            };
            let get_all_commits = match database::read_repository_commit(cloned_project.github.id) {
                Ok(x) => x,
                Err(_) => {
                    error!("Failed to read commits");
                    return;
                }
            };
            for repository_commit in get_all_commits {
                info!("Working on commit hash {}", &repository_commit.commit_hash);
                git_analyser::checkout_commit(&cloned_project, &repository_commit.commit_hash);
                std::thread::sleep(std::time::Duration::from_millis(1000));
                let changed_files = match database::read_commits_file(&repository_commit.commit_hash) {
                    Ok(files) => files,
                    Err(_) => {info!("Could not get any files for this commit"); continue;}
                };
                git_analyser::run_file_analysis(changed_files);
            }

        });
    }
}
