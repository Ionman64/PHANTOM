extern crate project_analyser;
extern crate fern;
#[macro_use]
extern crate log;
extern crate chrono;

use project_analyser::models::*;
use project_analyser::downloader;
use project_analyser::thread_helper::ThreadPool;
use project_analyser::git_analyser;
use std::io::ErrorKind;
use std::process::Command;
use std::path::{Path};
use std::fs;
use std::ops::Add;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

const THREAD_POOL_SIZE:usize = 75;
const ROOT_FOLDER:&str = "project_analyser";


fn main() {
    project_analyser::setup_logger().expect("Logger Setup Failed");
    setup_file_system();
    let repositories = get_all_repositories_from_filesystem();
    let thread_pool = ThreadPool::new(THREAD_POOL_SIZE);
    for project in repositories.into_iter().take(1) {
        thread_pool.execute(move || {
            let cloned_project = clone_project(project);
            if cloned_project.is_none() {
                return;
            }
            let cloned_project = cloned_project.unwrap();
            if !save_git_log_to_file(&cloned_project) {
                return;
            }
            if !parse_git_log(&cloned_project) {
                error!("Could not parse git log");
            }
            //generate_git_log_for_project(&cloned_project);
            //checkout_commits_for_project(&cloned_project);
        });
    }
}

pub fn get_git_log_file_path_as_string(cloned_project:&ClonedProject) -> String {
    let home_dir = downloader::get_home_dir_path().expect("Could not get home directory");
    let csv_path = Path::new(&home_dir)
        .join(ROOT_FOLDER)
        .join("git_log")
        .join(&cloned_project.github.id.to_string().add(".log"));
    csv_path.into_os_string().into_string().unwrap()
}

pub fn get_git_log_output_file_path_as_string(cloned_project:&ClonedProject) -> String {
    let home_dir = downloader::get_home_dir_path().expect("Could not get home directory");
    let csv_path = Path::new(&home_dir)
        .join(ROOT_FOLDER)
        .join("analysis_csv")
        .join(&cloned_project.github.id.to_string().add(".log"));
    csv_path.into_os_string().into_string().unwrap()
}

pub fn save_git_log_to_file(cloned_project: &ClonedProject) -> bool {
    Command::new("./scripts/save_git_log.sh").args(&[&cloned_project.input_log_path, &get_git_log_file_path_as_string(cloned_project)]).spawn().is_ok()
}

pub struct Commit {
    pub hash: String,
    pub author: String,
    pub date: String,
    pub comment: String,
}

pub fn parse_git_log(cloned_project: &ClonedProject) -> bool {
    let input_file = match File::open(get_git_log_file_path_as_string(&cloned_project)) {
        Ok(f) => f,
        Err(_) => {error!("could not read input_file");return false;}
    };
    let output_file = match File::create(get_git_log_output_file_path_as_string(&cloned_project)) {
        Ok(f) => f,
        Err(_) => {error!("could not read output_file");return false;}
    };
    let mut in_commit = false;
    let mut file_buf = BufReader::new(&input_file);
    for line_result in file_buf.lines() {
        let mut line = match line_result {
            Ok(x) => x,
            Err(_) => {return false;},
        };
        if line.len() == 0 {
            continue;
        }
        line = line.replace("\n", "");
        let first_word = line.split_whitespace().collect::<Vec<&str>>().get(0).unwrap();
        match first_word {
            &"commit" => {
                if in_commit {
                    in_commit = false;
                }
                else {
                    println!("{}", line.replace("commit ", ""));
                    in_commit = true;
                }
            },
            &"Author:" => {
                println!("{}", line.replace("Author: ", ""));
            },
            &"Date:" => {
                println!("{}", line.replace("Date: ", ""));
            },
            _ => {
                println!("Cannot Read this {}", line);
            }
        }

    }
    return true;
}

/*pub fn get_file_name(file: &CommitFile) -> String {
    return file.file_path.split(fs::MAIN_SEPARATOR).collect::<Vec<&str>>().last().unwrap().to_string();
}*/

fn setup_file_system() {
    let home_dir = downloader::get_home_dir_path().expect("Could not get home directory");
    let folders = vec!{"repos", "git_log", "analysis_csv", "feature_vectors"};
    for folder in folders {
        let project_path = Path::new(&home_dir)
            .join(String::from(ROOT_FOLDER))
            .join(String::from(folder));
        if !project_path.exists() {
            print!("Creating path {}...", &project_path.to_str().unwrap());
            match fs::create_dir_all(&project_path) {
                Err(_) => {panic!("ERROR (could not create {})", &project_path.to_str().unwrap())},
                Ok(_) => {println!("SUCCESS")}
            }
        }
    }
}

fn get_all_repositories_from_filesystem() -> Vec<GitRepository> {
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

fn generate_git_log_for_project(cloned_project: &ClonedProject) {
    match git_analyser::parse_git_log(&cloned_project) { // TODO rethink error messages
        Ok(_) => {}
        Err(ErrorKind::AlreadyExists) => {}
        Err(ErrorKind::InvalidData) => { error!("Invalid Data") }
        Err(ErrorKind::InvalidInput) => { error!("Invalid input when creating log") }
        Err(ErrorKind::Other) => { error!("Unknown error creating log") }
        Err(e) => { error!("Failed to generate git log for project {}: {:?}", &cloned_project.path, e); }
    };
}

