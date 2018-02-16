use super::*;
use schema::commit_file::dsl::*;
use diesel::result::Error;


pub fn create(conn: &PgConnection, entry: &[NewCommitFile]) -> DatabaseResult<usize> {
    match diesel::insert_into(commit_file)
        .values(entry)
        .execute(conn) {
            Ok(len) => Ok(len),
            Err(Error::QueryBuilderError(_)) => {info!("Could not build query"); return Err(ErrorKind::Other)},
            Err(Error::SerializationError(_)) => {info!("Database could not serialise a column"); return Err(ErrorKind::Other)},
            Err(Error::DatabaseError(_,_)) => {info!("Database Error: Possible Constraint Violation"); return Err(ErrorKind::AlreadyExists)},
            Err(_) => return Err(ErrorKind::Other),
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