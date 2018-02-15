use super::*;
use schema::repository_commit;
use models::{RepositoryCommit, NewRepositoryCommit};
use diesel::result::Error;

pub fn create(conn: &PgConnection, entry: Vec<NewRepositoryCommit>) -> DatabaseResult<usize> {
    match diesel::insert_into(repository_commit::table)
        .values(&entry)
        .execute(conn) {
            Ok(x) => return Ok(x),
            Err(Error::QueryBuilderError(_)) => {info!("Could not build query"); return Err(ErrorKind::Other)},
            Err(Error::SerializationError(_)) => {info!("Database could not serialise a column"); return Err(ErrorKind::Other)},
            //Err(Error::AlreadyInTransaction) => {info!("Transaction already open for client")},
            Err(Error::DatabaseError(_,_)) => {info!("Database Error: Possible Constraint Violation"); return Err(ErrorKind::Other)},
            Err(_) => return Err(ErrorKind::AlreadyExists),
        }
}

pub fn read(conn: &PgConnection, id: i64) -> DatabaseResult<Vec<RepositoryCommit>> {
    use schema::repository_commit::dsl::*;
    match repository_commit
        .filter(repository_id.eq(id))
        .order(commit_date.asc())
        .load::<RepositoryCommit>(conn) {
        Ok(x) => Ok(x),
        Err(_) => Err(ErrorKind::Other),
    }
}