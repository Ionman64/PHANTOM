use downloader::LinesResponse;
use models::ClonedProject;
use std::fs::File;
use std::io::{Write, BufWriter, BufRead, BufReader};
use std::process::Command;
use std::collections::HashMap;
use chrono::{NaiveDateTime, NaiveDate};
use std::io::ErrorKind;


fn generate_analysis_csv(project: &ClonedProject, datecount: HashMap<String, i32>) -> Result<(), ErrorKind> {
    let analysis_csv_file_output = File::create(&project.analysis_csv_file).unwrap();
    let mut bufwriter = BufWriter::new(analysis_csv_file_output);
    for (key, value) in datecount.iter() {
        let date = key;
        bufwriter.write_fmt(format_args!("{}, {}\n", date, value)).expect("Could not write analysis");
    }
    Ok(())
}

pub fn count_commits_per_day(cloned_project: &ClonedProject) -> Result<HashMap<NaiveDate, i16>, ErrorKind> {
    let mut date_count = HashMap::new();
    let git_log_lines = read_git_log_to_vec(&cloned_project.output_log_path).unwrap();

    for (i, line) in git_log_lines.response.iter().enumerate() {
        let timestamp: i64 = match line.parse() {
            Ok(val) => val,
            Err(e) => {
                error!("Could not parse timestamp '{}' in line {}. Timestamp was skipped. Error: {}",
                       line, i + 1, e);
                continue;
            }
        };
        let date = NaiveDateTime::from_timestamp(timestamp, 0).date();
        let count = date_count.entry(date).or_insert(0);
        *count += 1;
    }

    Ok(date_count)
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
    //TODO::change this to main separator
    let output_log_file = match File::create(&cloned_project.output_log_path) {
        Ok(file) => file,
        Err(_) => {
            warn!("Could not create cloned project");
            return Err(ErrorKind::Other);
        }
    };
    let mut bufwriter = BufWriter::new(&output_log_file);

    let command = match Command::new("git")
        .args(&["--git-dir", &cloned_project.input_log_path, "log", "--format=%ct"])
        .output() {
        Ok(output) => output,
        Err(_) => {
            warn!("Could not create git log");
            return Err(ErrorKind::Other);
        }
    };

    match bufwriter.write_all(&command.stdout) {
        Ok(_) => {}
        Err(_) => {
            warn!("could not write git log to file");
            return Err(ErrorKind::Other);
        }
    }
    bufwriter.flush().expect("Could not flush bufwriter");
    Ok(&cloned_project)
}
