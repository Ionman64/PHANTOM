use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::Command;
use std::str;
use std::io::ErrorKind;

pub struct LinesResponse {
    pub projects: Vec<GitHubProject>,
    pub skipped_lines: Option<Vec<u64>>
}

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
pub fn read_project_urls_from_file() -> Result<LinesResponse, ErrorKind> {
    let path = String::from("projects.csv");
    //TODO:: Fix below line to return it's error
    let csv_file = File::open(path).expect("Could not open urls file");
    let reader = BufReader::new(csv_file);
    let mut projects: Vec<GitHubProject> = Vec::new();
    let skip_rows = 1;
    let mut skipped_lines:Vec<u64> = Vec::new();
    let mut lineNum:u64 = 1;
    for (count, line) in reader.lines().enumerate().skip(skip_rows) {
        lineNum += 1;
        let str_line = match line {
            Ok(line) => line,
            Err(_) => {
                warn!("Could not read line {}", lineNum);
                skipped_lines.push(lineNum);
                continue;
            }
        };

        if character_count(&str_line, ',') == 0 {
            warn!("Does not contain expected comma character on line {}", lineNum);
            skipped_lines.push(lineNum);
            continue;
        }

        let columns: Vec<&str> = str_line.trim().split(',').collect();

        if columns.len() > 2 {
            let id: i64 = match columns.get(0).unwrap().parse() {
                Ok(id) => id,
                Err(_) => {
                    warn!("Could not parse id from CSV file");
                    continue;
                }
            };
            let url = columns.get(1).unwrap().to_string();
            projects.push(GitHubProject::new(id, url));
        } else {
            warn!("Err: Line {} is not formatted correctly and has been skipped.", lineNum);
            skipped_lines.push(lineNum);;
        }
    }
    Ok(LinesResponse {projects, skipped_lines:None})
}

///Counts the number of matching characters in a String
pub fn character_count(str_line: &String, matching_character: char) -> u64 {
    let mut count:u64 = 0;

    for character in str_line.chars() {
        if character == matching_character {
            count += 1;
        }
    }
    return count;
}

pub fn clone_project(project: GitHubProject) -> Result<ClonedProject, ErrorKind> {
    let home_path = get_home_dir_path().expect("Could not get home directory");
    let project_path = Path::new(&home_path)
        .join(String::from("project_analyser"))
        .join(String::from("repos"))
        .join(project.id.to_string());

    if !project_path.exists() {
        match fs::create_dir_all(&project_path) {
            Ok(_) => {
                info!("Project path created");
                Ok(())
            },
            Err(_) => {
                warn!("Could not create project directory");
                Err(ErrorKind::Other)
            }
        };
    }

    let cloned_project = ClonedProject::new(project, project_path);
    info!("Downloading {} from {}", &cloned_project.github.id, &cloned_project.github.url);
    Command::new("git")
        .args(&["clone", &cloned_project.github.url, &cloned_project.path])
        .output()
        .expect("Failed to clone");
    info!("Downloaded {} from {}", &cloned_project.github.id, &cloned_project.github.url);
    Ok(cloned_project)
}

pub fn get_home_dir_path() -> Result<String, ErrorKind> {
    let home_dir = match env::home_dir() {
        None => PathBuf::from(""),
        Some(path) => PathBuf::from(path),
    };
    match home_dir.into_os_string().into_string() {
        Ok(s) => Ok(s),
        Err(_) => {
            error!("Could not convert home dir into string.");
            return Err(ErrorKind::Other) //("Could not convert home dir into string");
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
        Ok(code_string) => match code_string[1..4].parse() {
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











