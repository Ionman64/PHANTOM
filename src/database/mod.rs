extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use models::*;
use std::env;
use std::io::ErrorKind;

type DatabaseResult<T> = Result<T, ErrorKind>;

mod git_repository;
mod repository_commit;
mod commit_file;

const MAX_QUERY_VALUES: usize = 65535;

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is undefined");
    establish_connection_with_url(database_url)
}

fn establish_connection_with_url(database_url: String) -> PgConnection {
    PgConnection::establish(&database_url).expect("Could not connect to database")
}

/* Create entries *********************************************************************************/
pub fn create_git_repository(project: NewGitRepository) -> DatabaseResult<GitRepository> {
    let conn = establish_connection();
    git_repository::create(&conn, project)
}

pub fn create_repository_commit(entry: Vec<NewRepositoryCommit>) -> DatabaseResult<usize> {
    let conn = establish_connection();

    /*
    let chunks = entry.chunks(MAX_QUERY_VALUES / NewRepositoryCommit::count_fields());
    for chunk in chunks {
        repository_commit::create(&conn, &chunk)?;
    }
    Ok(entry.len())
    */
    create_in_chunks(&repository_commit::create, &conn, entry)
}

pub fn create_commit_file(entry: Vec<NewCommitFile>) -> DatabaseResult<usize> {
    let conn = establish_connection();
    commit_file::create(&conn, entry)
}

fn create_in_chunks<T: FieldCountable>(fun: &Fn(&PgConnection, &[T]) -> DatabaseResult<usize>, conn: &PgConnection, entries: Vec<T>) -> DatabaseResult<usize> {
    let chunks = entries.chunks(MAX_QUERY_VALUES / T::count_fields());
    for chunk in chunks {
        fun(conn, chunk)?;
    }
    Ok(entries.len())
}

/* Read entries ***********************************************************************************/
pub fn read_git_repository(url: String) -> DatabaseResult<GitRepository> {
    let conn = establish_connection();
    git_repository::read(&conn, url)
}

pub fn read_repository_commit(id: i64) -> DatabaseResult<Vec<RepositoryCommit>> {
    let conn = establish_connection();
    repository_commit::read(&conn, id)
}

/* Update entries *********************************************************************************/

/* Delete entries *********************************************************************************/


/* Unit Tests *************************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;

    mod util {
        use super::*;

        pub fn database_url_var() -> String { String::from("DATABASE_URL") }

        pub fn testdatabase_url_var() -> String { String::from("TESTDATABASE_URL") }

        pub fn database_server_url_var() -> String { String::from("DBSERVER_URL") }

        pub fn setup_test_database() {
            use std::process::Command;
            dotenv().ok();
            let server_url = env::var(database_server_url_var()).unwrap();
            Command::new("./scripts/setup_test_db.sh").arg(server_url).arg("test_pa").output().expect("Could not setup test database.");
        }

        pub fn establish_test_connection() -> PgConnection {
            establish_connection_with_url(env::var(testdatabase_url_var()).unwrap())
        }
    }

    #[test]
    fn database_url_in_env() {
        dotenv().ok();
        assert!(env::var(util::database_url_var()).is_ok());
    }

    #[test]
    fn testdatabase_url_in_env() {
        dotenv().ok();
        assert!(env::var(util::testdatabase_url_var()).is_ok());
    }

    #[test]
    fn database_server_url_in_env() {
        dotenv().ok();
        assert!(env::var(util::database_server_url_var()).is_ok());
    }


    #[test]
    fn establish_connection_to_database() {
        dotenv().ok();
        establish_connection_with_url(env::var(util::database_url_var()).unwrap());
    }

    #[test]
    fn establish_connection_to_database2() {
        establish_connection();
    }


    #[test]
    #[ignore]
    fn establish_connection_to_test_database() {
        util::setup_test_database();
        dotenv().ok();
        util::establish_test_connection();
    }

    #[test]
    #[should_panic]
    fn establish_connection_with_wrong_connection_string() {
        dotenv().ok();
        establish_connection_with_url(String::from("not working url"));
    }

    #[test]
    #[ignore]
    fn create_new_git_repository() {
        util::setup_test_database();

        let conn = util::establish_test_connection();
        let url = String::from("https://github.com/new/repo");

        match git_repository::create(&conn, NewGitRepository { url: url.clone() }) {
            Ok(created_repository) => assert_eq!(url, created_repository.url),
            Err(_) => assert!(false),
        };
    }

    #[test]
    #[ignore]
    fn create_git_repository_with_url_that_exists_already() {
        util::setup_test_database();

        let conn = util::establish_test_connection();
        let url1 = String::from("https://github.com/new/repo");
        let url2 = url1.clone();
        let first = git_repository::create(&conn, NewGitRepository { url: url1 });
        let second = git_repository::create(&conn, NewGitRepository { url: url2 });

        assert!(first.is_ok());
        assert!(second.is_err());
    }

    #[test]
    #[ignore] // Must not run in parrellel with other
    fn read_git_repository_table_without_entries() {
        util::setup_test_database();

        let conn = util::establish_test_connection();
        let result = git_repository::read(&conn, String::from("https://github.com/new/repo"));

        assert!(result.is_err());
    }

    #[test]
    #[ignore]
    fn create_and_read_git_repository() {
        util::setup_test_database();

        let conn = util::establish_test_connection();
        let url = String::from("https://github.com/new/repo");
        let created = git_repository::create(&conn, NewGitRepository { url: url.clone() }).unwrap();
        let read = git_repository::read(&conn, url.clone()).unwrap();

        assert_eq!(created.id, read.id);
        assert_eq!(created.url, read.url);
    }

    #[test]
    #[ignore]
    fn create_many_and_read_many_git_repositories() {
        util::setup_test_database();
        let conn = util::establish_test_connection();

        let urls = vec!["one", "two", "three", "four", "five"];
        for url in urls.iter() {
            git_repository::create(&conn, NewGitRepository { url: url.to_string() }).unwrap();
        }

        for url in urls.into_iter() {
            let read = git_repository::read(&conn, String::from(url)).unwrap();
            assert_eq!(String::from(url), read.url);
        }
    }

    #[test]
    #[ignore]
    fn create_new_commit_frequency() {
        use chrono::{NaiveDate, NaiveDateTime};
        util::setup_test_database();
        let conn = util::establish_test_connection();

        let url = String::from("https://github.com/new/repo");
        let created_repository = git_repository::create(&conn, NewGitRepository { url: url.clone() }).unwrap();

        let repository_id = created_repository.id;
        let commit_date = NaiveDate::from_ymd(2018, 1, 1).and_hms(0, 0, 0);
        let frequency = 10;

        let created_frequency = commit_frequency::create(&conn, vec![CommitFrequency { repository_id, commit_date, frequency }]);
        assert!(created_frequency.is_ok());

        let created_frequency = created_frequency.unwrap();
        assert_eq!(repository_id, created_frequency.repository_id);
        assert_eq!(commit_date, created_frequency.commit_date);
        assert_eq!(frequency, created_frequency.frequency);
    }

    #[test]
    #[ignore]
    fn create_commit_frequencies_with_same_ids_and_dates() {
        use chrono::{NaiveDate, NaiveDateTime};
        util::setup_test_database();
        let conn = util::establish_test_connection();

        let url = String::from("https://github.com/new/repo1");
        let created_repository = git_repository::create(&conn, NewGitRepository { url: url.clone() }).unwrap();

        let repository_id = created_repository.id;
        let commit_date = NaiveDate::from_ymd(2018, 1, 1).and_hms(0, 0, 0);
        let frequency = 1;

        let frequency1 = CommitFrequency { repository_id, commit_date, frequency };
        let frequency2 = CommitFrequency { frequency: 2, ..frequency1 };

        let created1 = commit_frequency::create(&conn, vec![frequency1]);
        let created2 = commit_frequency::create(&conn, vec![frequency2]);

        assert!(created1.is_ok());
        assert!(created2.is_err());
    }

    #[test]
    #[ignore]
    fn create_commit_frequency_with_invalid_id() {
        use chrono::{NaiveDate, NaiveDateTime};
        util::setup_test_database();
        let conn = util::establish_test_connection();

        let url = String::from("https://github.com/new/repo");
        let created_repository = git_repository::create(&conn, NewGitRepository { url: url.clone() }).unwrap();

        let repository_id = created_repository.id + 1; // There should only be one valid id, by altering it must be invalid
        let commit_date = NaiveDate::from_ymd(2018, 1, 1).and_hms(0, 0, 0);
        let frequency = 10;

        assert!(commit_frequency::create(&conn, vec![CommitFrequency { repository_id, commit_date, frequency }]).is_err());
    }

    #[test]
    #[ignore]
    fn create_many_and_read_many_commit_frequencies() {
        use chrono::{NaiveDate, NaiveDateTime};
        util::setup_test_database();
        let conn = util::establish_test_connection();

        let urls = vec!["one", "two", "three", "four", "five"];
        let mut repositories: Vec<GitRepository> = Vec::new();
        for url in urls.iter() {
            repositories.push(git_repository::create(&conn, NewGitRepository { url: url.to_string() }).unwrap());
        }


        let commit_date1 = NaiveDate::from_ymd(2018, 1, 1).and_hms(0, 0, 0);
        let commit_date2 = NaiveDate::from_ymd(2018, 2, 2).and_hms(0, 0, 0);
        let commit_date3 = NaiveDate::from_ymd(2018, 3, 3).and_hms(0, 0, 0);

        for create_repository in repositories.iter() {
            commit_frequency::create(&conn, vec![CommitFrequency {
                repository_id: create_repository.id,
                commit_date: commit_date1.clone(),
                frequency: 10,
            }]);

            commit_frequency::create(&conn, vec![CommitFrequency {
                repository_id: create_repository.id,
                commit_date: commit_date2.clone(),
                frequency: 20,
            }]);

            commit_frequency::create(&conn, vec![CommitFrequency {
                repository_id: create_repository.id,
                commit_date: commit_date3.clone(),
                frequency: 30,
            }]);
        }

        for created_repository in repositories.iter() {
            assert_eq!(commit_frequency::read(&conn, created_repository.id, None).unwrap().len(), 3);
        }
    }

    #[test]
    #[ignore]
    fn create_many_commit_frequencies_and_read_back_ordered_by_date() {
        use chrono::{NaiveDate, NaiveDateTime};
        util::setup_test_database();
        let conn = util::establish_test_connection();

        let url = String::from("https://github.com/new/repo");
        let created_repository = git_repository::create(&conn, NewGitRepository { url: url.clone() }).unwrap();

        let commit_date1 = NaiveDate::from_ymd(2018, 1, 1).and_hms(0, 0, 0);
        let commit_date2 = NaiveDate::from_ymd(2018, 2, 2).and_hms(0, 0, 0);
        let commit_date3 = NaiveDate::from_ymd(2018, 3, 3).and_hms(0, 0, 0);
        let commit_date4 = NaiveDate::from_ymd(2018, 4, 4).and_hms(0, 0, 0);
        let commit_date5 = NaiveDate::from_ymd(2018, 5, 5).and_hms(0, 0, 0);


        // insert commit frequencies in order: 3, 1, 5, 4, 2
        commit_frequency::create(&conn, vec![CommitFrequency {
            repository_id: created_repository.id,
            commit_date: commit_date3.clone(),
            frequency: 30,
        }]);
        commit_frequency::create(&conn, vec![CommitFrequency {
            repository_id: created_repository.id,
            commit_date: commit_date1.clone(),
            frequency: 10,
        }]);
        commit_frequency::create(&conn, vec![CommitFrequency {
            repository_id: created_repository.id,
            commit_date: commit_date5.clone(),
            frequency: 50,
        }]);
        commit_frequency::create(&conn, vec![CommitFrequency {
            repository_id: created_repository.id,
            commit_date: commit_date4.clone(),
            frequency: 40,
        }]);
        commit_frequency::create(&conn, vec![CommitFrequency {
            repository_id: created_repository.id,
            commit_date: commit_date2.clone(),
            frequency: 20,
        }]);
        // read commit frequencies in order: 1, 2, 3, 4, 5
        let read_frequencies = commit_frequency::read(&conn, created_repository.id, None).unwrap();

        assert_eq!(read_frequencies.len(), 5);
        let read1 = &read_frequencies[0];
        let read2 = &read_frequencies[1];
        let read3 = &read_frequencies[2];
        let read4 = &read_frequencies[3];
        let read5 = &read_frequencies[4];

        assert_eq!(read1.commit_date, commit_date1);
        assert_eq!(read2.commit_date, commit_date2);
        assert_eq!(read3.commit_date, commit_date3);
        assert_eq!(read4.commit_date, commit_date4);
        assert_eq!(read5.commit_date, commit_date5);
    }
}

