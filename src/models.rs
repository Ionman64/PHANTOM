use super::schema::{git_repository, repository_commit, commit_file, file_analysis};
use chrono::NaiveDateTime;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use downloader::get_home_dir_path;
use std::ops::Add;

///
/// This macro implements 'count_fields()' for a public struct
///
macro_rules! make_fields_countable {
    (
        $(#[$m:meta])*
        pub struct $name:ident {
            $(pub $field_name:ident : $field_type:ty,)*
        }
    ) => {
        $(#[$m])*
        pub struct $name {
            $(pub $field_name : $field_type,)*

        }

        impl FieldCountable for $name {
            fn count_fields() -> usize {
                vec![$(stringify!($field_name)),*].len()
            }
        }
    };
}

pub trait FieldCountable {
    fn count_fields() -> usize;
}

#[derive(Queryable)]
pub struct GitRepository {
    pub id: i64,
    pub url: String,
}

make_fields_countable! {
    #[derive(Debug)]
    #[derive(Insertable)]
    #[table_name = "file_analysis"]
    pub struct FileAnalysis {
        pub file_id: i64,
        pub commit_hash: String,
        pub loc: i32,
    }
}

make_fields_countable! {
    #[derive(Debug)]
    #[derive(Insertable)]
    #[table_name = "commit_file"]
    pub struct NewCommitFile {
        pub commit_hash: String,
        pub repository_id: i64,
        pub file_path: String,
        pub action: String,
    }
}


#[derive(Debug)]
#[derive(Queryable)]
pub struct CommitFile {
    pub file_id: i64,
    pub commit_hash: String,
    pub repository_id: i64,
    pub file_path: String,
    pub action: String,
}


#[derive(Debug)]
#[derive(Queryable)]
pub struct RepositoryCommit {
    pub repository_id: i64,
    pub commit_hash: String,
    pub commit_date: NaiveDateTime,
}

make_fields_countable! {
    # [derive(Debug, Clone)]
    #[derive(Insertable)]
    # [table_name = "repository_commit"]
    pub struct NewRepositoryCommit {
        pub repository_id: i64,
        pub commit_hash: String,
        pub commit_date: NaiveDateTime,
    }
}

make_fields_countable! {
    #[derive(Insertable)]
    #[table_name = "git_repository"]
    pub struct NewGitRepository {
        pub url: String,
    }
}

impl NewRepositoryCommit {
    /// Helper function to create a new struct
    pub fn new(repository_id: i64, commit_date: NaiveDateTime, commit_hash: String) -> NewRepositoryCommit {
        // TODO Validate
        NewRepositoryCommit { repository_id, commit_date, commit_hash }
    }
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

impl CommitFile {
    pub fn get_abs_path(file: &CommitFile) -> PathBuf {
        let path =  Path::new(&get_home_dir_path().unwrap())
            .join("project_analyser")
            .join("repos")
            .join(file.repository_id.to_string())
            .join(file.file_path.to_string());
        println!("{} is at {}", file.file_id, path.as_os_str().to_str().unwrap());
        return path;
    }
    pub fn get_file_name(file: &CommitFile) -> String {
        return file.file_path.split(MAIN_SEPARATOR).collect::<Vec<&str>>().last().unwrap().to_string();
    }
}
