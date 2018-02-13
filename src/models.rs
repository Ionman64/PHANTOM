use super::schema::{git_repository, commit_frequency, repository_commit};
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct GitRepository {
    pub id: i64,
    pub url: String,
}

impl NewRepositoryCommit {
    /// Helper function to create a new struct
    pub fn new(repository_id:i64, commit_date:NaiveDateTime, commit_hash:String) -> NewRepositoryCommit {
        // TODO Validate
        NewRepositoryCommit  {repository_id, commit_date, commit_hash}
    }
}

#[derive(Queryable)]
#[derive(Debug)]
pub struct RepositoryCommit {
    pub commit_id: i64,
    pub repository_id: i64,
    pub commit_date: NaiveDateTime,
    pub commit_hash: String,
}

#[derive(Debug,Clone)]
#[derive(Insertable)]
#[table_name="repository_commit"]
pub struct NewRepositoryCommit {
    pub repository_id: i64,
    pub commit_date: NaiveDateTime,
    pub commit_hash: String,
}

#[derive(Queryable)]
#[derive(Debug)]
#[derive(Insertable)]
#[table_name="commit_frequency"]
pub struct CommitFrequency {
    pub repository_id: i64,
    pub commit_date: NaiveDateTime,
    pub frequency: i16,
}

#[derive(Insertable)]
#[table_name="git_repository"]
pub struct NewGitRepository {
    pub url: String
}

impl NewGitRepository {
    /// Helper function to create a new struct
    pub fn new(url: String) -> NewGitRepository {
        // TODO Validate
        NewGitRepository { url }
    }
}

pub struct ClonedProject {
    pub github: GitRepository,
    pub path: String,
    pub output_log_path: String,
    pub input_log_path: String,
    pub analysis_csv_file: String,
}

use std::path::{Path, PathBuf};
use downloader::get_home_dir_path;
use std::ops::Add;

impl ClonedProject {
    /// Helper function to create a new struct
    pub fn new(github: GitRepository, file_path: PathBuf) -> ClonedProject {
        // TODO Validate

        let csv_path = Path::new(&get_home_dir_path().unwrap())
            .join("project_analyser")
            .join("analysis")
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

