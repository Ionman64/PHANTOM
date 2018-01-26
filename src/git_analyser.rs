use downloader::{get_home_dir_path, ClonedProject};

use std::path::{PathBuf, Path};
use std::fs;
use std::fs::File;
use std::io::{Write, BufWriter, BufRead, BufReader};
use std::process::Command;
use std::collections::HashMap;
use chrono::NaiveDateTime;
use std::ops::Add;
use std::io::ErrorKind;


pub fn generate_analysis_csv(project: &ClonedProject, datecount: HashMap<String, i32>) -> Result<(), ErrorKind> {
    let csv_file_name = project.github.id.to_string().add(".csv");
    //TODO::handle get home path error
    let csv_path = Path::new(&get_home_dir_path().unwrap())
        .join("project_analyser")
        .join("analysis");
    fs::create_dir_all(&csv_path).expect("Could not create directories");
    let mut bufwriter = BufWriter::new(&log_file);
    for (key, value) in datecount.iter() {
        let date = key;
        bufwriter.write_fmt(format_args!("{}, {}\n", date, value));
    }
    Ok(())
}

pub fn count_commits_per_day(cloned_project: &ClonedProject) -> Result<HashMap<String, i32>, ErrorKind> {
    let mut date_count = HashMap::new();
    let git_log_lines = read_git_log_to_vec(&cloned_project.output_log_path).unwrap();
    for (i, line) in git_log_lines.iter().enumerate() {
        let timestamp: i64 = match line.parse() {
            Ok(val) => val,
            Err(e) => {
                error!(
                    "Could not parse timestamp '{}' in line {}. Timestamp was skipped. Error: {}",
                    line,
                    i + 1,
                    e
                );
                continue;
            }
        };
        let date = NaiveDateTime::from_timestamp(timestamp, 0)
            .date()
            .to_string();
        let count = date_count.entry(date).or_insert(0);
        *count += 1;
        //info!("Date: {} -> {}", date.date(), count);
    }
    Ok(date_count)
}

fn read_git_log_to_vec(filepath: &String) -> Result<Vec<String>, ErrorKind> {
    let file = File::open(filepath).expect("Git log not found");
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    let mut skipped_lines = 0;
    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(value) => lines.push(value),
            Err(e) => {
                warn!("Could not read line {} in git log. Err: {}", i + 1, e);
                skipped_lines += 1;
            },
        }
    }
    Ok(lines)
}

/// Generate a log by calling "git log" in the specified project directory.
/// Results with the path to the log file.
pub fn generate_git_log(cloned_project: &ClonedProject) -> Result<&ClonedProject, ErrorKind> {
    //TODO::change this to main separator
    let output_log_file = match File::create(&cloned_project.output_log_path) {
        Ok(file) => file,
        Err(_) => {
            warn!("Could not create cloned project");
            return Err(ErrorKind::Other)
        }
    };
    let mut bufwriter = BufWriter::new(&output_log_file);

    let command = match Command::new("git")
        .args(&["--git-dir", &cloned_project.input_log_path, "log", "--format=%ct"])
        .output() {
        Ok(output) => output,
        Err(_) => {
            warn!("Could not create git log");
            return Err(ErrorKind::Other)
        },
    };

    match bufwriter.write_all(&command.stdout) {
        Ok(_) => {},
        Err(_) => {
            warn!("could not write git log to file");
            return Err(ErrorKind::Other)
        },
    }
    bufwriter.flush();
    Ok(&cloned_project)
}

fn get_anaylsis_output_dir() -> PathBuf {
    let analysis_dir = Path::new(&get_home_dir_path())
        .join("project_analyser")
        .join("analysis");
    fs::create_dir_all(&analysis_dir).expect("Could not create directories");

    analysis_dir
}
