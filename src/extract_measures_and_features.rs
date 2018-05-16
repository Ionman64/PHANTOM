extern crate csv;
extern crate quickersort;

use std::fs;
use super::*;
use std::io::{Write};
use chrono::{NaiveDateTime, Timelike};
use std::cmp;
use chrono::Datelike;
use std::fs::{OpenOptions, File};
use std::path::{Path};
use self::csv::ReaderBuilder;




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

const PEAK_UP:i8 = 1;
const PEAK_DOWN:i8 = -1;
const PEAK_NONE:i8 = 0;

#[derive(Debug, Clone)]
struct UniqueDeveloperList {
    developers:Vec<String>,
}

impl UniqueDeveloperList {
    pub fn new() -> UniqueDeveloperList {
        UniqueDeveloperList {developers:Vec::new()}
    }
    pub fn addDeveloper(&mut self, developer:String) -> bool {
        if !self.developers.contains(&developer) {
            self.developers.push(developer);
            return true;
        }
        return false;
    }
    pub fn countDevelopers(&mut self) -> usize {
        self.developers.len()
    }
}

#[derive(Debug)]
struct FeatureVector {
    duration: usize,
    max_y: usize,
    max_y_pos: usize,
    mean_y: f64,
    sum_y: usize,
    q25: Option<f64>,
    q50: Option<f64>,
    q75: Option<f64>,
    std_y: Option<f64>,
    peak_down: Option<usize>,
    peak_none: Option<usize>,
    peak_up: Option<usize>,
    min_tbp_up: Option<usize>,
    avg_tbp_up: Option<f64>,
    max_tbp_up: Option<usize>,
    min_amplitude: Option<f64>,
    avg_amplitude: Option<f64>,
    max_amplitude: Option<f64>,
    min_ppd: Option<f64>,
    avg_ppd: Option<f64>,
    max_ppd: Option<f64>,
    min_npd: Option<f64>,
    avg_npd: Option<f64>,
    max_npd: Option<f64>,
    min_ps: Option<usize>,
    mean_ps: Option<f64>,
    max_ps: Option<usize>,
    sum_ps: Option<usize>,
    min_ns: Option<usize>,
    mean_ns: Option<f64>,
    max_ns: Option<usize>,
    sum_ns: Option<usize>,
    min_pg: Option<f64>,
    avg_pg: Option<f64>,
    max_pg: Option<f64>,
    min_ng: Option<f64>,
    avg_ng: Option<f64>,
    max_ng: Option<f64>,
    pg_count: Option<usize>,
    ng_count: Option<usize>,
}

impl FeatureVector {
    /// Helper function to create a new struct
    pub fn new() -> FeatureVector {
        FeatureVector {
            duration: 0,
            max_y: 0,
            max_y_pos: 0,
            mean_y: 0.0,
            sum_y: 0,
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

pub fn extract_from_directory(path_to_dir: String) -> bool {
    let log_folder_dir = fs::read_dir(path_to_dir).expect("Could not read projects dir");
    for log_file_path_result in log_folder_dir {
        let log_path = log_file_path_result.unwrap().path();
        match File::open(&log_path) {
            Ok(f) => f,
            Err(_) => {
                error!("Could not open log file at {:?}!", &log_path);
                continue;
            }
        };
        let feature_vectors = match extract_all_measures_from_file(&log_path, (&log_path).as_os_str().to_str().unwrap()) {
            None => {error!("Could not read feature vector in {:?}", &log_path); continue;},
            Some(fv) => {fv},
        };

        println!("{:?}", &log_path);
        for feature_vec in feature_vectors {
            println!("{:?}", feature_vec);
        }
    }
    true
}

fn extract_all_measures_from_file(log_file_path: &Path, file_name: &str) -> Option<Vec<FeatureVector>> {
    let mut csv_log_reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .double_quote(false)
        .flexible(true)
        .from_path(log_file_path).unwrap();

    let mut earliest_integration_date: i64 = 9999999999;
    let mut latest_integration_date: i64 = 0;
    let mut earliest_author_date: i64 = 9999999999;
    let mut latest_author_date: i64 = 0;
    for commit in csv_log_reader.records() {
        let commit = commit.unwrap();
        // reject csv with too few/many columnszoon
        if commit.len() != EXPECTED_CSV_COLUMNS {
            add_project_to_bad_log_file(&file_name);
            error!("Malformed line in {}", &file_name);
            return None;
        }
        // get earliest and latest integration date
        let current_integration_date: i64 = match commit.get(INTEGRATOR_DATE) {
            None => {
                add_project_to_bad_log_file(&file_name);
                error!("Malformed line in {}", &file_name);
                return None;
            },
            Some(value) => {
                value.parse().unwrap()
            },
        };
        earliest_integration_date = cmp::min(current_integration_date, earliest_integration_date);
        latest_integration_date = cmp::max(current_integration_date, latest_integration_date);
        // get earliest and latest author date
        let current_author_date: i64 = match commit.get(AUTHOR_DATE) {
            None => {
                add_project_to_bad_log_file(&file_name);
                error!("Malformed line in {}", &file_name);
                return None;
            },
            Some(value) => {
                value.parse().unwrap()
            },
        };
        earliest_author_date = cmp::min(current_author_date, earliest_author_date);
        latest_author_date = cmp::max(current_author_date, latest_author_date);
    }
    let earliest_integration_date = get_monday_timestamp(earliest_integration_date);
    let latest_integration_date = get_monday_timestamp(latest_integration_date);
    let earliest_author_date = get_monday_timestamp(earliest_author_date);
    let latest_author_date = get_monday_timestamp(latest_author_date);

    let total_weeks_integration = calculate_week_num(&earliest_integration_date, &latest_integration_date) + 1;
    let total_weeks_commits= calculate_week_num(&earliest_author_date, &latest_author_date) + 1;

    //Measures Taken
    let mut integration_frequency_timeseries = vec![0; total_weeks_integration];
    let mut integrator_activity_count = vec![UniqueDeveloperList::new(); total_weeks_integration];
    let mut integrator_activity_timeseries = vec![0; total_weeks_integration];

    let mut commit_frequency_timeseries = vec![0; total_weeks_commits];
    let mut author_activity_count = vec![UniqueDeveloperList::new(); total_weeks_commits];
    let mut author_activity_timeseries = vec![0; total_weeks_commits];

    let mut merge_frequency_timeseries = vec![0; total_weeks_integration];

    let mut csv_log_reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .double_quote(false)
        .flexible(true)
        .from_path(log_file_path).unwrap();


    for commit in csv_log_reader.records() {
        let commit = commit.unwrap();

        let current_integration_date: i64 = commit.get(INTEGRATOR_DATE).unwrap().parse().unwrap(); // unwrap because this has worked in the first for loop
        let integration_week_number = calculate_week_num(&earliest_integration_date, &current_integration_date);

        let current_author_date: i64 = commit.get(AUTHOR_DATE).unwrap().parse().unwrap(); // unwrap because this has worked in the first for loop
        let committer_week_number = calculate_week_num(&earliest_author_date, &current_author_date);


        //Integration_frequency
        integration_frequency_timeseries[integration_week_number] += 1;

        //Integrator_activity
        let integrator_email = String::from(commit.get(INTEGRATOR_EMAIL).unwrap());
        if integrator_activity_count[integration_week_number].addDeveloper(integrator_email) {
            integrator_activity_timeseries[integration_week_number] += 1;
        }

        //Commit_frequency
        commit_frequency_timeseries[committer_week_number] += 1;

        //author activity
        let author_email = String::from(commit.get(AUTHOR_EMAIL).unwrap());
        if author_activity_count[committer_week_number].addDeveloper(author_email) {
            author_activity_timeseries[committer_week_number] += 1;
        }


        //merge frequency
        let parent_hashes:String = String::from(commit.get(PARENT_HASHES).unwrap()); // unwrap because this has worked in the first for loop
        if super::character_count(&parent_hashes, ' ') > 0 {
            merge_frequency_timeseries[integration_week_number] += 1;
        }
    }
    let integration_frequency_feature_vector = calculate_feature_vector_from_time_series(&mut integration_frequency_timeseries);
    let integrator_activity_feature_vector = calculate_feature_vector_from_time_series(&mut integrator_activity_timeseries);
    let commit_frequency_feature_vector = calculate_feature_vector_from_time_series(&mut commit_frequency_timeseries);
    let author_activity_feature_vector = calculate_feature_vector_from_time_series(&mut author_activity_timeseries);
    let merge_frequency_feature_vector = calculate_feature_vector_from_time_series(&mut merge_frequency_timeseries);

    Some(vec![integration_frequency_feature_vector, integrator_activity_feature_vector, commit_frequency_feature_vector, author_activity_feature_vector, merge_frequency_feature_vector])
}

fn calculate_feature_vector_from_time_series(timeseries: &mut Vec<usize>) -> FeatureVector {
    let mut max_value = 0;
    let mut max_value_week = 0;
    let mut features = FeatureVector::new(); // TODO name should be "account_repo" (without ".log" at the end!)
    features.duration = timeseries.len();
    features.max_y = 0;
    features.max_y_pos = 0;
    for (current_week_index, current_value) in timeseries.iter().enumerate() {
        if current_value > &features.max_y {
            features.max_y = *current_value;
            features.max_y_pos = current_week_index + 1;
        }
        features.sum_y += current_value;
        // TODO If current_week_index > 0, then store gradient in a vector (see __extract_gradient_features__() in feature_extraction.py)
    }
    features.mean_y = features.sum_y as f64 / features.duration as f64;
    features.std_y = Some(standard_deviation(&timeseries));

    let (peaks, features) = detect_peaks_and_set_features(timeseries, features);

    features
}

fn standard_deviation(timeseries: &Vec<usize>) -> f64 {
    let mean = mean(&timeseries);
    let mut squared_diffs:Vec<f64> = Vec::new();
    for value in timeseries {
        let diff = (*value as f64 - mean);
        squared_diffs.push(diff * diff);
    }
    mean_f64(&squared_diffs).sqrt()
}

fn standard_deviation_f64(timeseries: &Vec<f64>) -> f64 {
    let mean = mean_f64(timeseries);
    let mut squared_diffs:Vec<f64> = Vec::new();
    for value in timeseries {
        let diff = (*value - mean);
        squared_diffs.push(diff * diff);
    }
    mean_f64(&squared_diffs).sqrt()
}

fn mean(timeseries:&Vec<usize>) -> f64 {
    let mut sum:usize = 0;
    for value in timeseries {
        sum += value;
    }
    (sum as f64 / timeseries.len() as f64)
}

fn mean_f64(timeseries:&Vec<f64>) -> f64 {
    let mut sum:f64 = 0.0;
    for value in timeseries {
        sum += value;
    }
    (sum / timeseries.len() as f64)
}

fn min_mean_max_sum(values: Vec<usize>) -> (usize, f64, usize, usize)  {
    let mut max = usize::min_value();
    let mut min = usize::max_value();
    let mut sum = 0;
    for value in values.iter() {
        max = std::cmp::max(*value, max);
        min = std::cmp::min(*value, min);
        sum += *value;
    }
    (min, (sum as f64 / values.len() as f64), max, sum)
}

fn min_mean_max_sum_f64(values: Vec<f64>) -> (f64, f64, f64, f64)  {
    let mut max = std::f64::MIN;
    let mut min = std::f64::MAX;
    let mut sum = 0.0;
    for value in values.iter() {
        max = max_value(max,*value);
        min = min_value(min,*value);
        sum += *value;
    }
    (min, (sum / values.len() as f64), max, sum)
}

fn max_value(v1:f64, v2:f64) -> f64 {
    if v1 == v2 {
        return v1;
    }
    if v1 > v2 {
        return v1;
    }
    v2
}

fn min_value(v1:f64, v2:f64) -> f64 {
    if v1 == v2 {
        return v1;
    }
    if v1 < v2 {
        return v1;
    }
    v2
}

fn calculate_gradient(v1:f64, v2:f64) -> f64 {
    // the difference on the x axis is expected to be always 1 week
    v2 - v1
}


///Detect all the peaks in a Vec<(f64)> and returns the indexes as a Vec<i64, PEAK>
/// #Example
/// let data_set: Vec<i64> = vec![0.0,1.0,0.0]
///
/// let result: Vec<i64, PEAK> = detect_all_peaks(data_set)
///
/// let (x, y) = result.get(0)
///
/// assert_eq!(x, 1)
///
/// assert_eq!(y, PEAK::UP)
fn detect_peaks_and_set_features(data_set: &mut Vec<usize>, mut features:FeatureVector) -> (Vec<i8>, FeatureVector) {
    let mut return_vector:Vec<i8> = vec![PEAK_NONE;data_set.len()];

    let mut current_gradient:f64 = 0.0;
    let mut previous_gradient_value:f64= 0.0;

    let mut positive_gradients:Vec<f64> = Vec::new();
    let mut negative_gradients:Vec<f64> = Vec::new();

    if data_set.len() <= 1 {
        return (return_vector, features);
    }

    let mut peak_up:usize = 0;
    let mut peak_down:usize = 0;
    let mut peak_none:usize = data_set.len();

    let mut current_seq = 0;

    let mut ps_sequence:Vec<usize> = Vec::new();
    let mut ns_sequence:Vec<usize> = Vec::new();

    let mean:f64 = mean(&data_set);

    let mut positive_deviations:Vec<f64> = Vec::new();
    let mut negative_deviations:Vec<f64> = Vec::new();

    let mut last_peak_up = 0;
    let mut last_peak_down = 0;

    let mut time_between_peaks_up:Vec<usize> = Vec::new();
    let mut time_between_peaks_down:Vec<usize> = Vec::new();

    let mut amplitudes:Vec<f64> = Vec::new();

    let mut index = 1;
    let array_length = data_set.len();
    let mut downward_trend = false;
    let mut upward_trend = false;
    let mut last_peak_down_value = data_set[0] as f64;
    while index < array_length {
        let previous = data_set[index-1] as f64;
        let current = data_set[index] as f64;

        if previous < current {
            current_seq += 1;
            upward_trend = true;
            if downward_trend {
                return_vector[index-1] = PEAK_DOWN;
                last_peak_down_value = previous;
                peak_down += 1;
                peak_none -= 1;
                ns_sequence.push(current_seq);
                negative_deviations.push(previous - mean);
                negative_gradients.push(current - previous_gradient_value);
                previous_gradient_value = current;
                current_seq = 0;
                time_between_peaks_down.push(index - last_peak_down);
                last_peak_down = index;
                downward_trend = false;
            }
        }
        if previous > current {
            downward_trend = true;
            if upward_trend {
                amplitudes.push(((previous - last_peak_down_value) / features.max_y as f64).abs());
                return_vector[index-1] = PEAK_UP;
                positive_deviations.push(previous - mean);
                peak_up += 1;
                peak_none -= 1;
                ps_sequence.push(current_seq);
                positive_gradients.push(current - previous_gradient_value);
                previous_gradient_value = current;
                current_seq = 0;
                time_between_peaks_up.push(index - last_peak_up);
                last_peak_up = index;
                upward_trend = false;
            }
        }
        index += 1;
    }
    if upward_trend {
        ns_sequence.push(current_seq);
    } else {
        ps_sequence.push(current_seq);
    }

    // TODO Refactor this code clone into a method
    features.pg_count = Some(positive_gradients.len());
    features.ng_count = Some(negative_gradients.len());
    let (min_pg, avg_pg, max_pg, _) = min_mean_max_sum_f64(positive_gradients.clone()); // TODO make work without cloning
    let (min_ng, avg_ng, max_ng, _) = min_mean_max_sum_f64(negative_gradients.clone()); // TODO make work without cloning
    features.min_pg = Some(min_pg);
    features.avg_pg = Some(avg_pg);
    features.max_pg = Some(max_pg);
    features.min_ng = Some(min_ng);
    features.avg_ng = Some(avg_ng);
    features.max_ng = Some(max_ng);




    features.peak_up = Some(peak_up);
    features.peak_down = Some(peak_down);
    features.peak_none = Some(peak_none);

    let (min_ps, mean_ps, max_ps, sum_ps) = min_mean_max_sum(ps_sequence);
    let (min_ns, mean_ns, max_ns, sum_ns) = min_mean_max_sum(ns_sequence);

    features.min_ps = Some(min_ps);
    features.mean_ps = Some(mean_ps);
    features.max_ps = Some(max_ps);
    features.sum_ps = Some(sum_ps);

    features.min_ns = Some(min_ns);
    features.mean_ns = Some(mean_ns);
    features.max_ns = Some(max_ns);
    features.sum_ns = Some(sum_ns);


    let (min_ppd, avg_ppd, max_ppd, _) = min_mean_max_sum_f64(positive_deviations);
    let (min_npd, avg_npd, max_npd, _) = min_mean_max_sum_f64(negative_deviations);

    features.min_ppd = Some(min_ppd);
    features.avg_ppd = Some(avg_ppd);
    features.max_ppd = Some(max_ppd);

    features.min_npd = Some(min_npd);
    features.avg_npd = Some(avg_npd);
    features.max_npd = Some(max_npd);

    features.pg_count = Some(positive_gradients.len());
    features.ng_count = Some(negative_gradients.len());

    let (min_pg, avg_pg, max_pg, _) = min_mean_max_sum_f64(positive_gradients);
    let (min_ng, avg_ng, max_ng, _) = min_mean_max_sum_f64(negative_gradients);

    features.min_pg = Some(min_pg);
    features.avg_pg = Some(avg_pg);
    features.max_pg = Some(max_pg);

    features.min_ng = Some(min_ng);
    features.avg_ng = Some(avg_ng);
    features.max_ng = Some(max_ng);

    let (min_tbp_up, avg_tbp_up, max_tbp_up, sum_tbp_up) = min_mean_max_sum(time_between_peaks_up);
    let (min_tbp_down, avg_tbp_down, max_tbp_down, sum_tbp_down) = min_mean_max_sum(time_between_peaks_down);

    features.min_tbp_up = Some(min_tbp_up);
    features.avg_tbp_up = Some(avg_tbp_up);
    features.max_tbp_up = Some(max_tbp_up);

    // TODO features.min_tbp_down = min_tbp_down;

    let (min_amp, avg_amp, max_amp, _) = min_mean_max_sum_f64(amplitudes);

    features.min_amplitude = Some(min_amp);
    features.avg_amplitude = Some(avg_amp);
    features.max_amplitude = Some(max_amp);


    data_set.sort();
    let (q25, q50, q75) = quartiles(data_set);

    features.q25 = Some(q25);
    features.q50 = Some(q50);
    features.q75 = Some(q75);

    //let (min_tbp_up, avg_tbp_up, max_tbp_up) = min_mean_max(ps_sequence);
    /*features.avg_tbp_up = Some(avg_tbp_up);
    features.max_tbp_up = Some(max_tbp_up);
    features.min_tbp_up = Some(min_tbp_up);*/
    (return_vector, features)
}

fn quartiles(timeseries:&Vec<usize>) -> (f64, f64, f64) {
    (find_quantile(timeseries, 0.25), find_quantile(timeseries, 0.5), find_quantile(timeseries, 0.75))
}

fn find_quantile(data_set:&Vec<usize>, quantile:f64) -> f64 {
    let length = (data_set.len() - 1) as f64;
    let upper_index = (length*quantile).ceil() as usize;
    let lower_index = (length*quantile).floor() as usize;
    let upper_value = data_set[upper_index];
    let lower_value = data_set[lower_index];
    (upper_value + lower_value) as f64 / 2.0
}

fn get_monday_timestamp(timestamp: i64) -> i64 {
    const SECONDS_PER_DAY:i64 = 86400;
    let naive_date = NaiveDateTime::from_timestamp(timestamp, 0);
    let seconds_from_midnight = naive_date.num_seconds_from_midnight() as i64;
    let days_from_monday = naive_date.weekday().num_days_from_monday() as i64;
    naive_date.timestamp() - (days_from_monday * SECONDS_PER_DAY) - seconds_from_midnight
}

fn calculate_week_num(base_time: &i64, week_time: &i64) -> usize{
    const SECS_IN_WEEK: i64 = 604800;
    return ((week_time - base_time) / SECS_IN_WEEK) as usize;
}

fn add_project_to_bad_log_file(project_file_name: &str) {
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(get_bad_log_file_as_string())
        .expect("Could not open bad log file");
    f.write_all(&project_file_name.as_bytes()).is_ok();
    f.write_all(b"\n").is_ok();
    f.flush().is_ok();
}

pub fn get_bad_log_file_as_string() -> String {
    let home_dir = get_home_dir_path().expect("Could not get home directory");
    let bad_log = Path::new(&home_dir)
        .join(super::get_root_folder())
        .join("bad_logs.log");
    bad_log.into_os_string().into_string().unwrap()
}