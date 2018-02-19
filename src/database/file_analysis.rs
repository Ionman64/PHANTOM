use super::*;
use schema::file_analysis;
use models::{FileAnalysis};
use diesel::result::Error;

pub fn create(conn: &PgConnection, entry: &[FileAnalysis]) -> DatabaseResult<usize> {
    match diesel::insert_into(file_analysis::table)
        .values(entry)
        .execute(conn) {
            Ok(x) => return Ok(x),
            Err(Error::QueryBuilderError(_)) => {info!("Could not build query"); return Err(ErrorKind::Other)},
            Err(Error::SerializationError(_)) => {info!("Database could not serialise a column"); return Err(ErrorKind::Other)},
            //Err(Error::AlreadyInTransaction) => {info!("Transaction already open for client")},
            Err(Error::DatabaseError(_,_)) => {info!("Database Error: Possible Constraint Violation"); return Err(ErrorKind::AlreadyExists)},
            Err(_) => return Err(ErrorKind::Other),
        }
}

/*pub fn read(conn: &PgConnection, id: i64, commit_hash:String) -> DatabaseResult<Vec<FileAnalysis>> {
    use schema::repository_commit::dsl::*;
    match repository_commit
        .filter(repository_id.eq(id))
        .filter(commit_hash.eq(commit_hash))
        .load::<FileAnalysis>(conn) {
        Ok(x) => Ok(x),
        Err(_) => Err(ErrorKind::Other),
    }
}*/