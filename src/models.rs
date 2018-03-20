use std::path::{Path, PathBuf};
use downloader::get_home_dir_path;
use std::ops::Add;
use downloader;

pub struct GitRepository {
    pub id: String,
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
}

impl ClonedProject {
    /// Helper function to create a new struct
    pub fn new(github: GitRepository, file_path: PathBuf) -> ClonedProject {
        ClonedProject {
            github,
            path: file_path.into_os_string().into_string().unwrap(),
        }
    }
}
