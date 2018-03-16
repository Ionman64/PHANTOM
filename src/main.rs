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
    execute();
}

fn get_all_repositories_from_filesystem() -> Vec<NewGitRepository> {
    match downloader::read_project_urls_from_file(String::from("projects.csv")) {
        Ok(project_struct) => {
            match project_struct.skipped_lines {
                None => {}
                Some(lines) => {
                    warn!("Read project from csv with success, but skipped lines: {:?}", lines);
                }
            }
            return project_struct.response;
        }
        Err(_) => {
            panic!("Failed to read any git repositories from filesystem");
        }
    }
}

fn clone_project(project: GitRepository) -> Option<ClonedProject> {
    let project_id = project.id.clone();
    match downloader::clone_project(project) {
        Ok(cloned_project) => Some(cloned_project),
        Err(_) => {
            error!("Failed to clone project {}.", project_id);
            return None;
        }
    }

}

fn get_all_commits_for_project(cloned_project: &ClonedProject) -> Option<Vec<RepositoryCommit>> {
    match database::read_repository_commit(cloned_project.github.id) {
        Ok(x) => Some(x),
        Err(_) => {
            error!("Failed to read commits");
            return None;
        }
    }
}

fn generate_git_log_for_project(cloned_project: &ClonedProject) {
    match git_analyser::generate_git_log(&cloned_project) { // TODO rethink error messages
        Ok(_) => {}
        Err(ErrorKind::AlreadyExists) => {}
        Err(ErrorKind::InvalidData) => { error!("Invalid Data") }
        Err(ErrorKind::InvalidInput) => { error!("Invalid input when creating log") }
        Err(ErrorKind::Other) => { error!("Unknown error creating log") }
        Err(e) => { error!("Failed to generate git log for project {}: {:?}", &cloned_project.path, e); }
    };
}

fn checkout_commits_for_project(cloned_project: &ClonedProject) {
    let all_commits = get_all_commits_for_project(cloned_project);
    if all_commits.is_none(){
        return;
    }
    let all_commits = all_commits.unwrap();
    for repository_commit in  all_commits {
        git_analyser::checkout_commit(&cloned_project, &repository_commit.commit_hash);
        let changed_files = match database::read_commits_file(&repository_commit.commit_hash) {
            Ok(files) => files,
            Err(_) => {info!("Could not get any files for this commit"); continue;}
        };
        git_analyser::run_file_analysis(changed_files); //TODO::Pass a function to this method to enable us to easily change which analysis we want to do
    }
}

fn execute() {
    let repositories = get_all_repositories_from_filesystem();

    let thread_pool = ThreadPool::new(75);
    for project in repositories.into_iter() {
        thread_pool.execute(move || {
            let cloned_project = clone_project(project);
            if cloned_project.is_none() {
                return;
            }
            let cloned_project = cloned_project.unwrap();
            generate_git_log_for_project(&cloned_project);
            //checkout_commits_for_project(&cloned_project);
        });
    }
}
