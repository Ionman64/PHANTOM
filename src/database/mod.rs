extern crate diesel;
extern crate dotenv;


use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use models::{NewGitRepository, GitRepository, CommitFrequency};
use std::env;
use std::io::ErrorKind;

type DatabaseResult<T> = Result<T, ErrorKind>;

mod commit_frequency;
mod git_repository;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Could not connect to database")
}

/* Create entries *********************************************************************************/
pub fn create_git_repository(project: NewGitRepository) -> DatabaseResult<GitRepository> {
    let conn = establish_connection();
    git_repository::create(&conn, project)
}

pub fn create_commit_frequency(entry: CommitFrequency) -> DatabaseResult<CommitFrequency> {
    let conn = establish_connection();
    commit_frequency::create(&conn, entry)
}

/* Create entries *********************************************************************************/
pub fn read_git_repository(url: String) -> DatabaseResult<GitRepository> {
    let conn = establish_connection();
    git_repository::read(&conn, url)
}

pub fn read_commit_frequency(id: i64, date: Option<NaiveDateTime>) -> DatabaseResult<Vec<CommitFrequency>> {
    let conn = establish_connection();
    commit_frequency::read(&conn, id, date)
}

/* Update entries *********************************************************************************/

/* Delete entries *********************************************************************************/


