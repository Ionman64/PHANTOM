use super::*;
use schema::git_repository::dsl::*;

pub fn create(conn: &PgConnection, entry: NewGitRepository) -> Result<GitRepository, ErrorKind> {
    match diesel::insert_into(git_repository)
        .values(&entry)
        .get_result(conn) {
        Ok(repository) => Ok(repository),
        Err(_) => Err(ErrorKind::AlreadyExists),
    }
}

pub fn read(conn: &PgConnection, url_entry: String) -> Result<GitRepository, ErrorKind>{
    match git_repository
        .filter(url.eq(&url_entry))
        .first(conn) {
        Ok(repository) => Ok(repository),
        Err(e) => {
            error!("Could not find {} in git_repository. Error:\n{}", url_entry, e);
            Err(ErrorKind::NotFound)
        }
    }
}