use chrono::NaiveDateTime;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use downloader::get_home_dir_path;
use std::ops::Add;

pub struct GitRepository {
    pub id: i64,
    pub url: String,
}

pub struct ClonedProject {
    pub github: GitRepository,
    pub path: String,
    pub output_log_path: String,
    pub input_log_path: String,
    pub analysis_csv_file: String,
}

impl ClonedProject {
    /// Helper function to create a new struct
    pub fn new(github: GitRepository, file_path: PathBuf) -> ClonedProject {
        // TODO Validate

        let csv_path = Path::new(&get_home_dir_path().unwrap())
            .join("project_analyser")
            .join("git_log")
            .join(github.id.to_string().add(".csv"));
        ClonedProject {
            github,
            analysis_csv_file: csv_path.into_os_string().into_string().unwrap(),
            output_log_path: file_path.join("pa_git.log").into_os_string().into_string().unwrap(),
            input_log_path: file_path.join(".git").into_os_string().into_string().unwrap(),
            path: file_path.into_os_string().into_string().unwrap(),
        }
    }
}
