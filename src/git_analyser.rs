use models::{ClonedProject};
use std::io::ErrorKind;
use std::process::Command;
use std::collections::HashMap;
use chrono::NaiveDateTime;
use std::io;
use std::path::{Path, PathBuf};

//Used for the File Analysis
use std::io::{BufReader,BufRead};
use std::fs::File;
use downloader;

#[derive(PartialEq)]
pub enum Git {
    DELETED,
    RENAMED,
    ADDED,
    MODIFIED,
    UNKNOWN
}

impl Git {
    pub fn to_string(&self) -> String {
        match *self {
            Git::DELETED => String::from("D"),
            Git::RENAMED => String::from("R"),
            Git::ADDED => String::from("A"),
            Git::MODIFIED => String::from("M"),
            Git::UNKNOWN => String::from("U")
        }
    }
    pub fn to_enum(action: &String) -> Git {
        match action.as_ref() {
            "D" => Git::DELETED,
            "A" => Git::ADDED,
            "R" => Git::RENAMED,
            "M" => Git::MODIFIED,
            _ => Git::UNKNOWN,
        }
    }
}

pub fn get_git_log_path(cloned_project: &ClonedProject) -> PathBuf {
    let home_path = downloader::get_home_dir_path().expect("Could not get home directory");

    Path::new(&home_path)
        .join(String::from("project_analyser"))
        .join(String::from("git_logs"))
        .join(cloned_project.github.id.to_string())
}



/// Generate a log by calling "git log" in the specified project directory.
/// Results with the path to the log file.
pub fn parse_git_log(cloned_project: &ClonedProject) -> Result<&ClonedProject, ErrorKind> {
    /*let mut date_count = HashMap::new();

    let git_files_and_dates_command = Command::new("git")
        .args(&["--git-dir", &cloned_project.analysis_csv_file, ]).spawn().is_err()
    let output = git_files_and_dates_command.stdout;
    let output_string = match String::from_utf8(output) {
        Ok(x) => x,
        Err(_) => return Err(ErrorKind::InvalidInput),
    };
    let mut current_commit_hash = String::from("");
    for line in output_string.split('\n').collect::<Vec<&str>>() {
        if String::from(line).len() == 0 {
            continue;
        }
        if line.contains(">>>>") {
            //Commit Hash and Timestamp
            let mut line_replace = str::replace(line, ">>>>", "");
            line_replace = str::replace(line_replace.as_str(), '"', "");
            let parts = line_replace.split(",").collect::<Vec<&str>>();
            current_commit_hash = String::from(parts[0]);
            let timestamp = match parts[1].parse() {
                Ok(x) => { x }
                Err(_) => { return Err(ErrorKind::InvalidData); }
            };
            let date = NaiveDateTime::from_timestamp(timestamp, 0);
            let count = date_count.entry(date).or_insert(0);
            *count += 1;
            //repository_commits.push(NewRepositoryCommit::new(cloned_project.github.id, date, current_commit_hash.clone()));
        } else {
            //File Name
            let words = line.split('\t').collect::<Vec<&str>>();
            let action = words.first().unwrap().to_string().chars().nth(0).unwrap().to_string();
            // Gets the last filepath, which is useful for files that have been rewritten
            if Git::to_enum(&action) == Git::RENAMED {
                let file_path = words.get(1).unwrap().to_string();
                let new_file_path = words.get(2).unwrap().to_string();
                commit_files.push(NewCommitFile {commit_hash: current_commit_hash.clone(), repository_id: cloned_project.github.id, file_path, action:Git::to_string(&Git::DELETED)});
                commit_files.push(NewCommitFile {commit_hash: current_commit_hash.clone(), repository_id: cloned_project.github.id, file_path:new_file_path, action:Git::to_string(&Git::ADDED)});
            } else {
                let file_path = words.get(1).unwrap().to_string();
            }

        }
    }*/
    Ok(cloned_project)
}
