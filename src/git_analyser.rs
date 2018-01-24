use downloader::get_home_dir_path;

use std::path::{Path, PathBuf};
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Write, BufWriter, BufRead, BufReader};
use std::process::Command;
use std::collections::HashMap;
use chrono::NaiveDateTime;

pub fn analyse_project(project_path: &Path) {
    let log_file = match generate_git_log(&project_path) {
        Ok(log) => {
            info!("Created log in {}", project_path.as_os_str().to_str().unwrap());
            log
        },
        Err(e) => {
            error!("Could not generate log file for project {}. Error: {}",
                   project_path.as_os_str().to_str().unwrap(),
                   e);
            return;
        },
    };

    let datecount = count_commits_per_day(&log_file);

    generate_analysis_csv(project_path, datecount)
}

fn generate_analysis_csv(project_path: &Path, datecount: HashMap<NaiveDateTime, i32>) {
    let project_name = project_path.file_name().unwrap().to_owned().into_string().unwrap();
    let csv_file_name =  project_name + &".csv".to_string();

    let csv_path = Path::new(&get_home_dir_path())
        .join("project_analyser")
        .join("analysis");
    fs::create_dir_all(&csv_path).expect("Could not create directories");

    let log_file = File::create(&csv_path.join(&csv_file_name)).unwrap();
    let mut bufwriter = BufWriter::new(&log_file);
    for (key, value) in datecount.iter() {
        let date = key.date().to_string();
        bufwriter.write_fmt(format_args!("{}, {}\n", date, value)).expect("Could not write file");
    }
}

fn count_commits_per_day(log_file: &Path) -> HashMap<NaiveDateTime, i32> {
    let mut date_count: HashMap<NaiveDateTime, i32> = HashMap::new();

    for (i, line) in read_git_log_to_vec(log_file).iter().enumerate() {
        let timestamp: i64 = match line.parse() {
            Ok(val) => val,
            Err(e) => {
                error!("Could not parse timestamp '{}' in line {}. Timestamp was skipped. Error: {}",
                       line,
                       i + 1, e);
                continue;
            }
        };
        let date = NaiveDateTime::from_timestamp(timestamp, 0);
        let count = date_count.entry(date).or_insert(0);
        *count += 1;
        //info!("Date: {} -> {}", date.date(), count);
    }

    date_count
}

fn read_git_log_to_vec(filepath: &Path) -> Vec<String> {
    let file = File::open(filepath).expect("Git log not found");
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(value) => lines.push(value),
            Err(e) => warn!("Could not read line {} in git log. Err: {}", i + 1, e),
        }
    }
    lines
}

/// Generate a log by calling "git log" in the specified project directory.
/// Results with the path to the log file.
fn generate_git_log(project_path: &Path) -> io::Result<PathBuf> {
    let log_file_path = &project_path.join("pa_git.log");
    let log_path_string = project_path.join(".git")
        .into_os_string()
        .into_string()
        .unwrap();

    let log_file = File::create(&log_file_path)?;
    let mut bufwriter = BufWriter::new(&log_file);

    let command = Command::new("git")
        .args(&["--git-dir", &log_path_string, "log", "--format=%ct"])
        .output()?;

    bufwriter.write_all(&command.stdout)?;
    bufwriter.flush()?;

    Ok(log_file_path.to_owned())
}