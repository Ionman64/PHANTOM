use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter};
use std::io::Write;
use std::process::Command;
use std::str;
use std::io;

/// Stores an id and a url to GitHub for a project
#[derive(Debug)]
pub struct GitHubProject {
    id: i64,
    url: String,
}

impl GitHubProject {
    /// Helper function to create a new struct
    pub fn new(id: i64, url: String) -> GitHubProject {
        // TODO Validate
        GitHubProject { id, url }
    }
}

/// Reads  the csv file "projects.csv" (see project root directory) and extracts the id and url for each row.
pub fn read_project_urls_from_file() -> Vec<GitHubProject> {
    let path = String::from("projects.csv");
    let csvfile = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            error!("Could not open file with project urls");
            panic!("Could not open file with project urls");
        },
    };

    let reader = BufReader::new(csvfile);

    let mut projects: Vec<GitHubProject> = Vec::new();
    let skip_rows = 1;
    for (counter, line) in reader.lines().enumerate().skip(skip_rows) {
        let str_line = match line {
            Ok(line) => line,
            Err(e) => {
                warn!("Could not read line {}", counter + skip_rows);
                continue;
            },
        };
        let columns: Vec<&str> = str_line.trim().split(',').collect();

        if columns.len() > 2 {
            let id: i64 = columns.get(0).unwrap().parse().unwrap();
            let url = columns.get(1).unwrap().to_string();

            projects.push(GitHubProject::new(id, url));
        } else {
            warn!("Err: Line {} is not formatted correctly and has been skipped.", counter + skip_rows);
        }
    }

    projects
}

pub fn clone_project(project: &GitHubProject) {
    let path_to_home = get_home_dir_path();
    let project_path = Path::new(&path_to_home)
        .join(String::from("project_analyser"))
        .join(String::from("out"))
        .join(project.id.to_string());

    if !project_path.exists() {
        fs::create_dir_all(&project_path);
    }
    info!("Downloading {} from {}", &project.id, &project.url);
    let project_path_string = project_path.clone().into_os_string().into_string().unwrap();
    let log_path_output = project_path.clone().join("pa_git_log.log").into_os_string().into_string().unwrap();
    let clone_command = Command::new("git")
        .args(&["clone", &project.url, &project_path_string])
        .output()
        .expect("Failed to clone");
    info!("Downloaded {} from {}", &project.id, &project.url);

    match generate_git_log(&project_path) {
        Ok(_) => info!("Created log for in {}", &log_path_output),
        Err(e) => error!("Could not write log file to {}:{}", &log_path_output, e),
    };
}
fn generate_git_log(project_path: &Path) -> Result<(), io::Error> {
    let log_path_string = project_path.clone().join(".git").into_os_string().into_string().unwrap();
    let file = File::create(&project_path.join("pa_log.txt"))?;
    let mut bufwriter = BufWriter::new(file);

    let command = Command::new("git")
        .args(&["--git-dir", &log_path_string , "log", "--format=%ct"])
        .output()
        .expect("Failed to create project log");

    bufwriter.write_all(&command.stdout)?;
    bufwriter.flush()?;

    Ok(())
}

fn get_home_dir_path() -> String {
    let home_dir = match env::home_dir() {
        None => PathBuf::from(""),
        Some(path) => PathBuf::from(path),
    };
    match home_dir.into_os_string().into_string() {
        Ok(s) => s,
        Err(e) => {
            error!("Could not convert home dir into string.");
            panic!("Could not convert home dir into string.");
        },
    }
}











