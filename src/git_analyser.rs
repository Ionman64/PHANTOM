use models::{ClonedProject, NewRepositoryCommit, NewCommitFile};
use std::io::ErrorKind;
use std::process::Command;
use std::collections::HashMap;
use chrono::NaiveDateTime;
use database;

/// Generate a log by calling "git log" in the specified project directory.
/// Results with the path to the log file.
pub fn generate_git_log(cloned_project: &ClonedProject) -> Result<&ClonedProject, ErrorKind> {
    let mut repository_commits: Vec<NewRepositoryCommit> = Vec::new();
    let mut date_count = HashMap::new();
    let git_files_and_dates_command = match Command::new("git")
        .args(&["--git-dir", &cloned_project.input_log_path, "log", "--name-status", "--format=\">>>>%H,%ct\""])
        .output() {
        Ok(output) => output,
        Err(_) => {
            warn!("Could not create git log");
            return Err(ErrorKind::InvalidInput);
        }
    };
    let output = git_files_and_dates_command.stdout;
    let output_string = match String::from_utf8(output) {
        Ok(x) => x,
        Err(_) => return Err(ErrorKind::InvalidInput),
    };
    let mut skip = false;
    let mut commit_files:Vec<NewCommitFile> = Vec::new();
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
            repository_commits.push(NewRepositoryCommit::new(cloned_project.github.id, date, current_commit_hash.clone()));
        } else {
            //File Name
            let words = line.split('\t').collect::<Vec<&str>>();
            let action = words.first().unwrap().to_string().chars().nth(0).unwrap().to_string();
            let file_path = words.last().unwrap().to_string();
            let temp_commit_file = NewCommitFile {commit_hash: current_commit_hash.clone(), repository_id: cloned_project.github.id, file_path, action};
            commit_files.push(temp_commit_file);

        }
    }
    match database::create_repository_commit(repository_commits) {
        Ok(x) => { info!("{} rows inserted into database: repository_id {}", x, &cloned_project.github.id) }
        Err(ErrorKind::AlreadyExists) => { info!("{} already exists in database", &cloned_project.github.id) }
        Err(ErrorKind::Other) => { info!("Other Error when inserting {} into database", &cloned_project.github.id); }
        Err(_) => { info!("Unknown Error when inserting {} into database", &cloned_project.github.id) }
    };
    match database::create_commit_file(commit_files) {
        Ok(x) => { info!("{} rows inserted into database: repository_id {}", x, &cloned_project.github.id) }
        Err(ErrorKind::AlreadyExists) => { info!("{} commit file already exists in database", &cloned_project.github.id) }
        Err(ErrorKind::Other) => { info!("Other Error when inserting {} into database", &cloned_project.github.id); }
        Err(_) => { info!("Unknown Error when inserting {} into database", &cloned_project.github.id) }
    };
    Ok(cloned_project)
}

pub fn checkout_commit(cloned_project: &ClonedProject, commit_hash: &String) -> Result<bool, ErrorKind> {
    let git_checkout_cmd = match Command::new("git")
        .args(&["--git-dir", &cloned_project.input_log_path, "checkout", &commit_hash, "-q"])
        .output() {
        Ok(output) => output,
        Err(_) => {
            warn!("Could not checkout {}: repository id {}", &commit_hash, &cloned_project.github.id);
            return Err(ErrorKind::InvalidInput);
        }
    };
    let output = git_checkout_cmd.stdout;
    if output.len() > 0 {
        let output_string = match String::from_utf8(output) {
            Ok(x) => x,
            Err(_) => return Err(ErrorKind::InvalidInput),
        };
        error!("Git checkout returned error {}", output_string);
        return Err(ErrorKind::InvalidData);
    }
    Ok(true)
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn read_commits_per_day_correct_url() {
        let github_project = GitRepository { id: -2, url: String::from("https://github.com/bitcoin/bitcoin") };
        let home_path = get_home_dir_path().expect("Could not get home directory");
        let project_path = Path::new(&home_path)
            .join(String::from("project_analyser"))
            .join(String::from("repos"))
            .join(github_project.id.to_string());
        let cloned_project = ClonedProject::new(github_project, project_path);
        let result = match count_commits_per_day(&cloned_project) {
            Ok(date_count) => true,
            Err(_) => false,
        };
        assert!(result);
    }
}
