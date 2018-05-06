use std::fs;
use super::*;
use std::io::{BufReader, BufRead, Write, BufWriter};
use chrono::{NaiveDateTime, Weekday, Duration};
use std::cmp;
use chrono::Datelike;
use std::ops::Sub;
use std::fs::{OpenOptions, File};
use std::path::{Path, PathBuf};

extern crate csv;


const EXPECTED_CSV_COLUMNS: usize = 8;
//CSV COLUMNS
const COMMIT_HASH: usize = 0;
const PARENT_HASHES: usize = 1;
const AUTHOR_NAME: usize = 2;
const AUTHOR_EMAIL: usize = 3;
const AUTHOR_DATE: usize = 4;
const INTEGRATOR_NAME: usize = 5;
const INTEGRATOR_EMAIL: usize = 6;
const INTEGRATOR_DATE: usize = 7;

struct FeatureVector {
    project_name: String,
    duration: Option<f32>,
    may_y: Option<f32>,
    max_y_pos: Option<f32>,
    mean_y: Option<f32>,
    sum_y: Option<f32>,
    q25: Option<f32>,
    q50: Option<f32>,
    q75: Option<f32>,
    std_y: Option<f32>,
    peak_down: Option<f32>,
    peak_none: Option<f32>,
    peak_up: Option<f32>,
    min_tbp_up: Option<f32>,
    avg_tbp_up: Option<f32>,
    max_tbp_up: Option<f32>,
    min_amplitude: Option<f32>,
    avg_amplitude: Option<f32>,
    max_amplitude: Option<f32>,
    min_ppd: Option<f32>,
    avg_ppd: Option<f32>,
    max_ppd: Option<f32>,
    min_npd: Option<f32>,
    avg_npd: Option<f32>,
    max_npd: Option<f32>,
    min_ps: Option<f32>,
    mean_ps: Option<f32>,
    max_ps: Option<f32>,
    sum_ps: Option<f32>,
    min_ns: Option<f32>,
    mean_ns: Option<f32>,
    max_ns: Option<f32>,
    sum_ns: Option<f32>,
    min_pg: Option<f32>,
    avg_pg: Option<f32>,
    max_pg: Option<f32>,
    min_ng: Option<f32>,
    avg_ng: Option<f32>,
    max_ng: Option<f32>,
    pg_count: Option<f32>,
    ng_count: Option<f32>,
}

impl FeatureVector {
    /// Helper function to create a new struct
    pub fn new(name: String) -> FeatureVector {
        FeatureVector {
            project_name: name,
            duration: None,
            may_y: None,
            max_y_pos: None,
            mean_y: None,
            sum_y: None,
            q25: None,
            q50: None,
            q75: None,
            std_y: None,
            peak_down: None,
            peak_none: None,
            peak_up: None,
            min_tbp_up: None,
            avg_tbp_up: None,
            max_tbp_up: None,
            min_amplitude: None,
            avg_amplitude: None,
            max_amplitude: None,
            min_ppd: None,
            avg_ppd: None,
            max_ppd: None,
            min_npd: None,
            avg_npd: None,
            max_npd: None,
            min_ps: None,
            mean_ps: None,
            max_ps: None,
            sum_ps: None,
            min_ns: None,
            mean_ns: None,
            max_ns: None,
            sum_ns: None,
            min_pg: None,
            avg_pg: None,
            max_pg: None,
            min_ng: None,
            avg_ng: None,
            max_ng: None,
            pg_count: None,
            ng_count: None,
        }
    }
}

pub fn extract_from_directory(path_to_dir: String) {
    let log_folder_dir = fs::read_dir(path_to_dir).expect("Could not read projects dir");
    for log_file_path_result in log_folder_dir {
        let log_path = log_file_path_result.unwrap().path();
        let log_file = match File::open(&log_path) {
            Ok(f) => f,
            Err(_) => {
                error!("Could not open log file at {:?}!", &log_path);
                continue;
            }
        };
        extract_all_measures_from_file(&log_path, (&log_path).as_os_str().to_str().unwrap());
    }
}

fn extract_all_measures_from_file(log_file_path: &Path, file_name: &str) -> Option<FeatureVector> {
    let integration_frequency: Vec<i64> = Vec::new();
    println!("Looking for earliest commit");
    let mut earliest_commit: usize = 9999999999;
    //let file_lines = BufReader::new(log_file).lines();
    //for (line_num, line) in file_lines.enumerate() {
    //    let csv_row = line.unwrap();
    //    let columns: Vec<&str> = csv_row.split(",").collect();

    use self::csv::ReaderBuilder;
    let mut csv_log_reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .double_quote(false)
        .flexible(true)
        .from_path(log_file_path).unwrap();

    for commit in csv_log_reader.records() {
        //let (hash, parents, author_name, author_email, author_date, committer_name, committer_email, committer_date): (String, String, String, String, usize, String, String, usize) = commit.unwrap();
        let commit = commit.unwrap();
        if commit.len() != EXPECTED_CSV_COLUMNS {
            add_project_to_bad_log_file(&file_name);
            error!("Malformed line in {}", &file_name);
            return None;
        }
        let new_commit_date: usize = match commit.get(INTEGRATOR_DATE) {
            None => {
                add_project_to_bad_log_file(&file_name);
                error!("Malformed line in {}", &file_name);
                return None;
            },
            Some(value) => {
                value.parse().unwrap()
            },
        };
        earliest_commit = cmp::min(new_commit_date, earliest_commit);

        if file_name == "/home/joshua/Documents/backups/logs/sample/testlog.log" {
            println!("{:?}", commit);
        }

    }
    let naive_date = NaiveDateTime::from_timestamp(earliest_commit as i64, 0);
    while naive_date.weekday() != Weekday::Mon {
        naive_date.sub(Duration::days(1));
        break;
    }
    println!("earliest_commit {} timestamp {}", earliest_commit, naive_date.timestamp());

    return Some(FeatureVector::new(String::from("Dummy")));

    /*
    for (line_num, line) in file_lines.enumerate() {
        let csv_row = line.unwrap();
        let columns: Vec<&str> = csv_row.split(",").collect();
        if columns.len() != EXPECTED_CSV_COLUMNS {
            add_project_to_bad_log_file(&file_name);
            error!("Malformed line in {}", &file_name);
        }
        let new_commit_string = columns.get(INTEGRATOR_DATE).unwrap();
        let new_commit_date: usize = new_commit_string.parse().expect("Could not parse to usize");

    }
    println!("found earliest commit: {}", earliest_commit);
    */
}

fn calculate_week_num(base_time: &usize, week_time: &usize) -> usize {
    const SECS_IN_WEEK: usize = 604800;
    return (week_time - base_time) / SECS_IN_WEEK;
}

fn add_project_to_bad_log_file(project_file_name: &str) {
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(get_bad_log_file_as_string())
        .expect("Could not open bad log file");
    f.write_all(&project_file_name.as_bytes());
    f.write_all(b"\n");
    f.flush();
}

pub fn get_bad_log_file_as_string() -> String {
    let home_dir = get_home_dir_path().expect("Could not get home directory");
    let bad_log = Path::new(&home_dir)
        .join(super::get_root_folder())
        .join("bad_logs.log");
    bad_log.into_os_string().into_string().unwrap()
}