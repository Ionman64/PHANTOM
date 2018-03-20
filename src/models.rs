use std::path::{Path, PathBuf};
use downloader::get_home_dir_path;
use std::ops::Add;
use downloader;

pub struct GitRepository {
    pub id: usize,
    pub url: String,
}

/*impl GitRepository {
    pub fn new(url:String) -> GitRepository {
        let home_dir = downloader::get_home_dir_path().expect("Could not get home directory");
        let csv_path = Path::new(&home_dir)
            .join(ROOT_FOLDER)
            .join("git_log")
            .join(&cloned_project.github.url.to_string()
                .replace("https://github.com/", "")
                .replace("/", "_")
                .add(".log"));
        csv_path.into_os_string().into_string().unwrap()
    }
}*/

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
