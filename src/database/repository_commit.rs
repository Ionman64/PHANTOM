use super::*;
use schema::repository_commit;
use models::RepositoryCommit;
use models::NewRepositoryCommit;

pub fn create(conn: &PgConnection, entry: Vec<NewRepositoryCommit>) -> DatabaseResult<usize> {
    match diesel::insert_into(repository_commit::table)
        .values(&entry)
        .execute(conn) {
            Ok(x) => Ok(x),
            Err(_) => return Err(ErrorKind::AlreadyExists),
        }
}

/*pub fn read(conn: &PgConnection, id: i64, date: Option<NaiveDateTime>) -> DatabaseResult<Vec<CommitFrequency>> {
    use schema::commit_frequency::dsl::*;
    let results: Vec<RepositoryCommit> = match repository_commit
        .filter(repository_id.eq(id))
        .order(commit_date.asc())
        .load::<CommitFrequency>(conn) {
        Ok(frequencies) => frequencies,
        Err(_) => return Err(ErrorKind::Other),
    };

    match date {
        None => return Ok(results),
        Some(date) => return Ok(results
            .into_iter()
            .filter(|entry| entry.commit_date == date)
            .collect()),
    }
}*/