use super::*;
use schema::git_repository::dsl::*;

pub fn create(conn: &PgConnection, entry: NewGitRepository) -> DatabaseResult<GitRepository> {
    match diesel::insert_into(git_repository)
        .values(&entry)
        .get_result(conn) {
        Ok(repository) => Ok(repository),
        Err(_) => Err(ErrorKind::AlreadyExists),
    }
}

pub fn read(conn: &PgConnection, url_entry: String) -> DatabaseResult<GitRepository>{
    match git_repository
        .filter(url.eq(&url_entry))
        .first(conn) {
        Ok(repository) => Ok(repository),
        Err(e) => Err(ErrorKind::NotFound),
    }
}