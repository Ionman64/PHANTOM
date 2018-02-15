use super::*;
use schema::commit_file::dsl::*;

pub fn create(conn: &PgConnection, entry: Vec<NewCommitFile>) -> DatabaseResult<usize> {
    match diesel::insert_into(commit_file)
        .values(&entry)
        .execute(conn) {
        Ok(x) => Ok(x),
        Err(_) => Err(ErrorKind::AlreadyExists),
    }
}

/*pub fn read(conn: &PgConnection, url_entry: String) -> DatabaseResult<GitRepository>{
    match git_repository
        .filter(url.eq(&url_entry))
        .first(conn) {
        Ok(repository) => Ok(repository),
        Err(_) => Err(ErrorKind::NotFound),
    }
}*/