use downloader::LinesResponse;
use models::{ClonedProject, CommitFrequency, NewRepositoryCommit};
use std::fs::File;
use std::io::{Write, BufWriter, BufRead, BufReader};
use std::process::Command;
use std::collections::HashMap;
use chrono::{NaiveDateTime, NaiveDate};
use std::io::ErrorKind;
use database;


fn generate_analysis_csv(project: &ClonedProject, datecount: HashMap<String, i32>) -> Result<(), ErrorKind> {
    let analysis_csv_file_output = File::create(&project.analysis_csv_file).unwrap();
    let mut bufwriter = BufWriter::new(analysis_csv_file_output);
    for (key, value) in datecount.iter() {
        let date = key;
        bufwriter.write_fmt(format_args!("{}, {}\n", date, value)).expect("Could not write analysis");
    }
    Ok(())
}

fn read_git_log_to_vec(filepath: &String) -> Result<LinesResponse<String>, ErrorKind> {
    let file = File::open(filepath).expect("Git log not found");
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    let mut skipped_lines: Vec<u32> = Vec::new();
    let mut line_num: u32 = 0;
    for line in reader.lines() {
        line_num += 1;
        match line {
            Ok(value) => lines.push(value),
            Err(e) => {
                warn!("Could not read line {} in git log. Err: {}", line_num, e);
                skipped_lines.push(line_num);
            }
        }
    }
    Ok(LinesResponse { response: lines, skipped_lines: Some(skipped_lines) })
}

/// Generate a log by calling "git log" in the specified project directory.
/// Results with the path to the log file.
pub fn generate_git_log(cloned_project: &ClonedProject) -> Result<&ClonedProject, ErrorKind> {
    let mut repository_commits:Vec<NewRepositoryCommit> = Vec::new();
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
    let commits_to_files: Vec<(String, Vec<String>)> = Vec::new();
    for line in output_string.split('\n').collect::<Vec<&str>>() {
        if line.contains(">>>>") {
            //Commit Hash and Timestamp
            let mut line_replace = str::replace(line, ">>>>", "");
            line_replace = str::replace(line_replace.as_str(), '"', "");
            let parts = line_replace.split(",").collect::<Vec<&str>>();
            let commit_hash = String::from(parts[0]);
            let timestamp = match parts[1].parse() {
                Ok(x) => {x},
                Err(_) => {return Err(ErrorKind::InvalidData)}
            };
            let date = NaiveDateTime::from_timestamp(timestamp, 0);
            let count = date_count.entry(date).or_insert(0);
            *count += 1;
            repository_commits.push(NewRepositoryCommit::new(cloned_project.github.id, date, commit_hash));
        }
        else {
            //File Name
            //println!("{}", line);
        }
    }
    let mut index = 0;
    let jump = 21845;
    let length = repository_commits.len();
    while index < length {
        let mut tempVec: Vec<NewRepositoryCommit> = Vec::new();
        for repository_commit in repository_commits.clone().into_iter().skip(index).take(jump) {
            tempVec.push(repository_commit);
        }
        index = index + jump;
        println!("{} at index {}", &cloned_project.github.id, jump);
        match database::create_repository_commit(&tempVec) {
            Ok(x) => { info!("{} rows inserted into database: repository_id {}", x, &cloned_project.github.id) },
            Err(ErrorKind::AlreadyExists) => { info!("{} already exists in database", &cloned_project.github.id) },
            Err(ErrorKind::Other) => { info!("Other Error when inserting {} into database", &cloned_project.github.id); },
            Err(_) => { info!("Unknown Error when inserting {} into database", &cloned_project.github.id) },
        };
        //Ok(&cloned_project)
    }
    let mut commit_frequencies:Vec<CommitFrequency> = Vec::new();
    for (commit_date, frequency) in date_count.into_iter() {
        let commit_frequency = CommitFrequency {repository_id:cloned_project.github.id, commit_date, frequency};
        commit_frequencies.push(commit_frequency);
    }
    match database::create_commit_frequencies(commit_frequencies) {
        Ok(_) => {},
        Err(ErrorKind::AlreadyExists) => {return Err(ErrorKind::AlreadyExists)},
        Err(_) => {return Err(ErrorKind::Other)}
    }
    Ok(cloned_project)
}
fn print_commits(id: &i64, commits: &Vec<NewRepositoryCommit>) {
    use downloader::get_home_dir_path;
    use std::path::Path;
    use std::fs;
    use std::ops::Add;
    let file_path = Path::new(&get_home_dir_path().unwrap())
        .join("project_analyser")
        .join(id.to_string().add(".sql"));
    let file = File::create(file_path).unwrap();
    let mut bufwriter = BufWriter::new(file);
    for commit in commits.into_iter() {
        bufwriter.write_fmt(format_args!("INSERT INTO repository_commit (repository_id, commit_hash, commit_date) VALUES ('{}','{}','{}');\n", commit.repository_id, commit.commit_hash, commit.commit_date));
    }
}

#[cfg(tests)]
mod tests {
    use super::*;
    #[test]
    fn read_commits_per_day_correct_url() {
        let github_project = GitRepository {id:7, url:String::from("https://github.com/bitcoin/bitcoin")};
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
