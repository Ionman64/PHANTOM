use super::*;
use schema::commit_frequency;
use models::CommitFrequency;

pub fn create(conn: &PgConnection, entry: CommitFrequency) -> DatabaseResult<CommitFrequency> {
    match diesel::insert_into(commit_frequency::table)
        .values(&entry)
        .get_result(conn)
        {
            Ok(frequency) => Ok(frequency),
            Err(_) => Err(ErrorKind::AlreadyExists),
        }
}

pub fn read(conn: &PgConnection, id: i64, date: Option<NaiveDateTime>) -> DatabaseResult<Vec<CommitFrequency>> {
    use schema::commit_frequency::dsl::*;
    let results: Vec<CommitFrequency> = match commit_frequency
        .filter(repository_id.eq(id))
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
}