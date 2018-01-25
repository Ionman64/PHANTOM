use downloader::{get_home_dir_path, ClonedProject};

use std::path::{PathBuf, Path};
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Write, BufWriter, BufRead, BufReader};
use std::process::Command;
use std::collections::HashMap;
use chrono::NaiveDateTime;
use std::ops::Add;

pub fn analyse_project(cloned_project: &ClonedProject) {
    let cloned_project = match generate_git_log(&cloned_project) {
        Ok(log) => {
            info!("Created log in {}", &cloned_project.path);
            log
        }
        Err(e) => {
            error!("Could not generate log file for project {}. Error: {}", &cloned_project.path, e);
            return;
        }
    };
    let datecount = count_commits_per_day(&cloned_project);
    generate_analysis_csv(&cloned_project, datecount)
}

fn generate_analysis_csv(project: &ClonedProject, datecount: HashMap<String, i32>) {
    let csv_file_name = project.github.id.to_string().add(".csv");
    let csv_path = get_anaylsis_output_dir().join(csv_file_name);

    let log_file = File::create(&csv_path).expect("Could not create file");

    let mut bufwriter = BufWriter::new(&log_file);
    for (key, value) in datecount.iter() {
        let date = key;
        bufwriter.write_fmt(format_args!("{}, {}\n", date, value))
            .expect("Could not write file");
    }
}

fn count_commits_per_day(cloned_project: &ClonedProject) -> HashMap<String, i32> {
    let mut date_count = HashMap::new();

    for (i, line) in read_git_log_to_vec(&cloned_project.output_log_path).iter().enumerate() {
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

    date_count
}

fn read_git_log_to_vec(filepath: &String) -> Vec<String> {
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
fn generate_git_log(cloned_project: &ClonedProject) -> io::Result<&ClonedProject> {
    //TODO::change this to main separator
    let output_log_file = File::create(&cloned_project.output_log_path)?;
    let mut bufwriter = BufWriter::new(&output_log_file);

    let command = Command::new("git")
        .args(&["--git-dir", &cloned_project.input_log_path, "log", "--format=%ct"])
        .output()?;

    bufwriter.write_all(&command.stdout)?;
    bufwriter.flush()?;

    Ok(cloned_project)
}

fn get_anaylsis_output_dir() -> PathBuf {
    let analysis_dir = Path::new(&get_home_dir_path())
        .join("project_analyser")
        .join("analysis");
    fs::create_dir_all(&analysis_dir).expect("Could not create directories");

    analysis_dir
}
