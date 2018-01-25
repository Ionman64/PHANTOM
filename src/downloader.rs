use git_analyser;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::Command;
use std::str;

/// Stores an id and a url to GitHub for a project
pub struct GitHubProject {
    pub id: i64,
    pub url: String,
}

pub struct ClonedProject {
    pub github: GitHubProject,
    pub path: String,
    pub output_log_path: String,
    pub input_log_path: String,
}

impl GitHubProject {
    /// Helper function to create a new struct
    pub fn new(id: i64, url: String) -> GitHubProject {
        // TODO Validate
        GitHubProject { id, url }
    }
}

impl ClonedProject {
    /// Helper function to create a new struct
    pub fn new(github: GitHubProject, file_path: PathBuf) -> ClonedProject {
        // TODO Validate
        ClonedProject {
            github,
            output_log_path: file_path.join("pa_git.log").into_os_string().into_string().unwrap(),
            input_log_path: file_path.join(".git").into_os_string().into_string().unwrap(),
            path: file_path.into_os_string().into_string().unwrap(),
        }
    }
}

/// Reads  the csv file "projects.csv" (see project root directory) and extracts the id and url for each row.
pub fn read_project_urls_from_file() -> Vec<GitHubProject> {
    let path = String::from("projects.csv");
    let csv_file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            error!("Could not open file with project urls");
            panic!("Could not open file with project urls");
        }
    };

    let reader = BufReader::new(csv_file);

    let mut projects: Vec<GitHubProject> = Vec::new();
    let skip_rows = 1;
    for (counter, line) in reader.lines().enumerate().skip(skip_rows) {
        let str_line = match line {
            Ok(line) => line,
            Err(_) => {
                warn!("Could not read line {}", counter + skip_rows);
                continue;
            }
        };
        let columns: Vec<&str> = str_line.trim().split(',').collect();

        if columns.len() > 2 {
            let id: i64 = columns.get(0).unwrap().parse().unwrap();
            let url = columns.get(1).unwrap().to_string();

            projects.push(GitHubProject::new(id, url)); //TODO: give the correct project path
        } else {
            warn!("Err: Line {} is not formatted correctly and has been skipped.", counter + skip_rows);
        }
    }

    projects
}


pub fn clone_project(project: GitHubProject) {
    let home_path = get_home_dir_path();
    let project_path = Path::new(&home_path)
        .join(String::from("project_analyser"))
        .join(String::from("repos"))
        .join(project.id.to_string());

    if !project_path.exists() {
        fs::create_dir_all(&project_path).expect("Could not create directories");
    }
    let cloned_project = ClonedProject::new(project, project_path);

    info!("Downloading {} from {}", &cloned_project.github.id, &cloned_project.github.url);
    Command::new("git")
        .args(&["clone", &cloned_project.github.url, &cloned_project.path, "-q"])
        .output()
        .expect("Failed to execute git clone");

    info!("Downloaded {} from {}", &cloned_project.github.id, &cloned_project.github.url);

    git_analyser::analyse_project(&cloned_project); // TODO spawn in new thread
}

pub fn get_home_dir_path() -> String {
    let home_dir = match env::home_dir() {
        None => PathBuf::from(""),
        Some(path) => PathBuf::from(path),
    };
    match home_dir.into_os_string().into_string() {
        Ok(s) => s,
        Err(_) => {
            error!("Could not convert home dir into string.");
            panic!("Could not convert home dir into string.");
        }
    }
}

/// Checks whether the url exists using curl.
fn check_url_http_code(expected_code: i32, url: &str) -> Result<(), ()> {
    // curl -s -o /dev/null-I  -I -w "%{http_code}"
    let curl = Command::new("curl")
        .args(&["-s", "-o", "/dev/null", "-I", "-w", "\"%{http_code}\"", url])
        .output()
        .expect("Could not run curl");

    let http_code = utf8_to_http_code(curl.stdout)?;

    if http_code == expected_code {
        Ok(())
    } else {
        warn!("Http code does not match. Found {}, Expected {} for url {}", http_code, expected_code, url);
        Err(())
    }
}


/// Tries to parse the specified data into a string and then into an integer
fn utf8_to_http_code(data: Vec<u8>) -> Result<i32, ()> {
    match String::from_utf8(data) {
        Ok(code_string) => match code_string.parse() {
            Ok(code_i32) => Ok(code_i32),
            Err(e) => {
                error!("Could not parse http code '{}' into int. Treating url as not existent. Err: {}", code_string, e);
                Err(())
            }
        },
        Err(e) => {
            error!("Could not create string from curl's output. Treating url as not existent. Err: {}", e);
            Err(())
        }
    }
}











