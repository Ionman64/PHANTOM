extern crate project_analyser;
extern crate fern;
#[macro_use]
extern crate log;

use project_analyser::models::*;
use project_analyser::downloader;
use project_analyser::thread_helper::ThreadPool;
use std::process::Command;
use std::path::{Path};
use std::fs;
use std::ops::Add;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::io::ErrorKind;


const THREAD_POOL_SIZE:usize = 75;
const ROOT_FOLDER:&str = "project_analyser";
const PROJECTS_FILE:&str = "dataset.csv";


fn main() {
    project_analyser::setup_logger().expect("Logger Setup Failed");
    setup_file_system();
    let thread_pool = ThreadPool::new(THREAD_POOL_SIZE);
    let f = match File::open(PROJECTS_FILE) {
        Ok(f) => f,
        Err(_) => {panic!("Could not open projects file!");}
    };
    for (line_num, line) in BufReader::new(f).lines().skip(1).enumerate() {
        let str_line = match line {
            Ok(line) => line,
            Err(_) => {
                warn!("Could not read line {}", &line_num);
                continue;
            }
        };

        let project = match extract_git_repo_from_line(line_num, str_line) {
            Ok(x) => x,
            Err(_) => {continue;},
        };

        // ------------
        let home_dir = downloader::get_home_dir_path().expect("Could not get home directory");
        let id = project.id.to_owned();
        let csv_path = Path::new(&home_dir)
            .join(ROOT_FOLDER)
            .join("analysis_csv")
            .join(id.add(".csv"));
        if csv_path.exists() {
            println!("Skipped project with id {}", project.id);
            continue
        }
        // ------------

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
                return;
            }
        });
    }
}

fn character_count(str_line: &String, matching_character: char) -> u32 {
    let mut count: u32 = 0;

    for character in str_line.chars() {
        if character == matching_character {
            count += 1;
        }
    }
    return count;
}

pub fn extract_git_repo_from_line(line_num: usize, str_line: String) -> Result<GitRepository, ErrorKind> {
    if character_count(&str_line, ',') == 0 {
        warn!("Does not contain expected comma character on line {}", &line_num);
        return Err(ErrorKind::InvalidInput);
    }

    let columns: Vec<&str> = str_line.trim().split(',').collect();
    if columns.len() <= 2 {
        warn!("Err: Line {} is not formatted correctly and has been skipped.", &line_num);
        return Err(ErrorKind::InvalidInput);
    }
    let mut url_context = columns.get(0).unwrap().to_string();
    url_context.replace("https://github.com/", "");
    let mut full_url = String::from("https://github.com/").add(&url_context);
    Ok(GitRepository {id:url_context.replace("/", "_"), url:full_url})
}

pub fn get_git_log_file_path_as_string(cloned_project:&ClonedProject) -> String {
    let home_dir = downloader::get_home_dir_path().expect("Could not get home directory");
    let id = cloned_project.github.id.to_owned();
    let csv_path = Path::new(&home_dir)
        .join(ROOT_FOLDER)
        .join("git_log")
        .join(id.add(".log"));
    csv_path.into_os_string().into_string().unwrap()
}

pub fn get_git_folder_from_project_as_string(cloned_project:&ClonedProject) -> String {
    let home_dir = downloader::get_home_dir_path().expect("Could not get home directory");
    let id = cloned_project.github.id.to_owned();
    let csv_path = Path::new(&home_dir)
        .join(ROOT_FOLDER)
        .join("repos")
        .join(id);
    csv_path.into_os_string().into_string().unwrap()
}

pub fn get_git_log_output_file_path_as_string(cloned_project:&ClonedProject) -> String {
    let home_dir = downloader::get_home_dir_path().expect("Could not get home directory");
    let id = cloned_project.github.id.to_owned();
    let csv_path = Path::new(&home_dir)
        .join(ROOT_FOLDER)
        .join("analysis_csv")
        .join(id.add(".csv"));
    csv_path.into_os_string().into_string().unwrap()
}
pub fn save_git_log_to_file(cloned_project: &ClonedProject) -> bool {
    Command::new("./scripts/save_git_log.sh").args(&[get_git_folder_from_project_as_string(cloned_project), get_git_log_file_path_as_string(cloned_project)]).output().is_ok()
}

pub fn parse_git_log(cloned_project: &ClonedProject) -> bool {
    let mut date_hashmap = HashMap::new();
    let input_file = match File::open(get_git_log_file_path_as_string(&cloned_project)) {
        Ok(f) => f,
        Err(_) => {error!("could not read input_file");return false;}
    };
    for (i, line_result) in BufReader::new(input_file).lines().enumerate() {
        let line = match line_result {
            Ok(x) => x,
            Err(_) => {error!("Could not read line {} of git log: Skipping Project {}", i, &cloned_project.github.id); return false;},
        };
        let count = date_hashmap.entry(line).or_insert(0);
        *count += 1;
    }
    let mut output_file = match File::create(get_git_log_output_file_path_as_string(&cloned_project)) {
        Ok(f) => f,
        Err(_) => {error!("could not read output_file");return false;}
    };
    for (key, value) in date_hashmap {
        output_file.write(format!("{},{}\n", key, value).as_bytes()).unwrap();
    }
    match output_file.sync_data() {
        Ok(_) => {},
        Err(_) => {error!("Could not sync file data for project {}", cloned_project.github.id)},
    };
    match output_file.flush() {
        Ok(_) => {},
        Err(_) => {error!("Could not flush file data for project {}", cloned_project.github.id)},
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

fn clone_project(project: GitRepository) -> Option<ClonedProject> {
    let id = project.id.clone();
    match downloader::clone_project(project) {
        Ok(cloned_project) => Some(cloned_project),
        Err(_) => {
            error!("Failed to clone project {}.", id);
            return None;
        }
    }
}

