use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ffi::OsString;


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
    let p = Path::new(&path_to_home)
        .join(String::from("project_analyser"))
        .join(String::from("out"))
        .join(project.id.to_string());

    if !p.exists() {
        fs::create_dir_all(p);
    }
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











