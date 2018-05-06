use std::fs;
use super::*;
use std::io::{ BufReader, BufRead, Write, BufWriter};
use chrono::{NaiveDateTime, Weekday, Duration};
use std::cmp;
use chrono::Datelike;
use std::ops::Sub;
use std::fs::{OpenOptions, File};
use std::path::{Path, PathBuf};

const EXPECTED_CSV_COLUMNS:usize = 8;
//CSV COLUMNS
const COMMIT_HASH:usize = 0;
const PARENT_HASHES:usize = 1;
const AUTHOR_NAME:usize = 2;
const AUTHOR_EMAIL:usize = 3;
const AUTHOR_DATE:usize = 4;
const INTEGRATOR_NAME:usize = 5;
const INTEGRATOR_EMAIL:usize = 6;
const INTEGRATOR_DATE:usize = 7;

struct FeatureVector {
    project_name:String,
    duration:usize,
    may_y:usize,
    max_y_pos:usize,
    mean_y:usize,
    sum_y:usize,
    q25:usize,
    q50:usize,
    q75:usize,
    std_y:usize,
    peak_down:usize,
    peak_none:usize,
    peak_up:usize,
    min_tbp_up:usize,
    avg_tbp_up:usize,
    max_tbp_up:usize,
    min_amplitude:usize,
    avg_amplitude:usize,
    max_amplitude:usize,
    min_ppd:usize,
    avg_ppd:usize,
    max_ppd:usize,
    min_npd:usize,
    avg_npd:usize,
    max_npd:usize,
    min_ps:usize,
    mean_ps:usize,
    max_ps:usize,
    sum_ps:usize,
    min_ns:usize,
    mean_ns:usize,
    max_ns:usize,
    sum_ns:usize,
    min_pg:usize,
    avg_pg:usize,
    max_pg:usize,
    min_ng:usize,
    avg_ng:usize,
    max_ng:usize,
    pg_count:usize,
    ng_count:usize,
}

impl FeatureVector {
    /// Helper function to create a new struct
    pub fn new(name: String) -> FeatureVector {
        FeatureVector {
            project_name:name,
            duration:0,
            may_y:0,
            max_y_pos:0,
            mean_y:0,
            sum_y:0,
            q25:0,
            q50:0,
            q75:0,
            std_y:0,
            peak_down:0,
            peak_none:0,
            peak_up:0,
            min_tbp_up:0,
            avg_tbp_up:0,
            max_tbp_up:0,
            min_amplitude:0,
            avg_amplitude:0,
            max_amplitude:0,
            min_ppd:0,
            avg_ppd:0,
            max_ppd:0,
            min_npd:0,
            avg_npd:0,
            max_npd:0,
            min_ps:0,
            mean_ps:0,
            max_ps:0,
            sum_ps:0,
            min_ns:0,
            mean_ns:0,
            max_ns:0,
            sum_ns:0,
            min_pg:0,
            avg_pg:0,
            max_pg:0,
            min_ng:0,
            avg_ng:0,
            max_ng:0,
            pg_count:0,
            ng_count:0,
        }
    }
}

pub fn extract_from_directory(path_to_dir: String) {
    let log_folder_dir = fs::read_dir(path_to_dir).expect("Could not read projects dir");
    for log_file_path_result in log_folder_dir {
        let log_path = log_file_path_result.unwrap().path();
        let f = match File::open(log_path.clone()) {
            Ok(f) => f,
            Err(_) => {error!("Could not open log file at {:?}!", &log_path);continue;}
        };
        let integration_frequency:Vec<i64> = Vec::new();
        println!("Looking for earliest commit");
        let mut earliest_commit:usize = 9999999999;
        let file_lines = BufReader::new(f).lines();
        for (line_num, line) in file_lines.enumerate() {
            let csv_row = line.unwrap();
            let columns: Vec<&str> = csv_row.split(",").collect();
            if columns.len() != EXPECTED_CSV_COLUMNS {
                add_project_to_bad_log_file(&log_path.as_os_str().to_str().unwrap());
                error!("Malformed line in {}", &log_path.as_os_str().to_str().unwrap());
            }
            let new_commit_string = columns.get(INTEGRATOR_DATE).unwrap();
            let new_commit_date: usize = new_commit_string.parse().expect("Could not parse to usize");
            earliest_commit = cmp::min(new_commit_date, earliest_commit);
        }
        let naive_date = NaiveDateTime::from_timestamp(earliest_commit as i64, 0);
        while naive_date.weekday() != Weekday::Mon {
            naive_date.sub(Duration::days(1));
            break;
        }
        println!("earliest_commit {} timestamp {}", earliest_commit, naive_date.timestamp());

        return;

        for (line_num, line) in file_lines.enumerate() {
            let csv_row = line.unwrap();
            let columns: Vec<&str> = csv_row.split(",").collect();
            if columns.len() != EXPECTED_CSV_COLUMNS {
                add_project_to_bad_log_file(&log_path.as_os_str().to_str().unwrap());
                error!("Malformed line in {}", &log_path.as_os_str().to_str().unwrap());
            }
            let new_commit_string = columns.get(INTEGRATOR_DATE).unwrap();
            let new_commit_date: usize = new_commit_string.parse().expect("Could not parse to usize");

        }
        println!("found earliest commit: {}", earliest_commit);
    }
}

fn calculate_week_num(base_time:&usize, week_time:&usize) -> usize {
    const SECS_IN_WEEK:usize = 604800;
    return (week_time - base_time) / SECS_IN_WEEK;
}

fn add_project_to_bad_log_file(project_file_name:&str) {
    let mut f = OpenOptions::new().append(true).open(get_bad_log_file_as_string()).expect("Could not open bad log file");
    f.write_all(&project_file_name.as_bytes());
    f.write_all(b"\n");
    f.flush();
}

pub fn get_bad_log_file_as_string() -> String {

    let home_dir = get_home_dir_path().expect("Could not get home directory");
    let bad_log = Path::new(&home_dir)
        .join(super::get_root_folder())
        .join("bad_logs.txt");
    bad_log.into_os_string().into_string().unwrap()
}