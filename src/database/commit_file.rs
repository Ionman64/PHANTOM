use super::*;
use schema::commit_file::dsl::*;
use diesel::result::Error;


pub fn create(conn: &PgConnection, entry: &[NewCommitFile]) -> DatabaseResult<Vec<CommitFile>> {
    match diesel::insert_into(commit_file)
        .values(entry)
        .get_results(conn) {
            Ok(results) => Ok(results),
            //Err(Error::NotFound(_)) => {info!("No rows were inserted into the database"); return Err(ErrorKind::NotFound)}
            Err(Error::QueryBuilderError(_)) => {info!("Could not build query"); return Err(ErrorKind::Other)},
            Err(Error::SerializationError(_)) => {info!("Database could not serialise a column"); return Err(ErrorKind::Other)},
            Err(Error::DatabaseError(_,_)) => {info!("Database Error: Possible Constraint Violation"); return Err(ErrorKind::AlreadyExists)},
            Err(_) => return Err(ErrorKind::Other),
        }
}

pub fn read(conn: &PgConnection, commit_hash_string: &String) -> DatabaseResult<Vec<CommitFile>> {
    match commit_file
        .filter(commit_hash.eq(commit_hash_string))
        .get_results(conn) {
            Ok(results) => Ok(results),
            //Err(Error::NotFound(_)) => {info!("No rows were inserted into the database"); return Err(ErrorKind::NotFound)}
            Err(Error::QueryBuilderError(_)) => {info!("Could not build query"); return Err(ErrorKind::Other)},
            Err(Error::SerializationError(_)) => {info!("Database could not serialise a column"); return Err(ErrorKind::Other)},
            Err(Error::DatabaseError(_,_)) => {info!("Database Error: Possible Constraint Violation"); return Err(ErrorKind::AlreadyExists)},
            Err(_) => return Err(ErrorKind::Other),
        }
}