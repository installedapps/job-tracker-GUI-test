use crate::{Job, Status};

pub fn row_to_job(row: &rusqlite::Row<'_>) -> Result<Job, rusqlite::Error> {
    let status_value: String = row.get(3)?;
    let status = Status::from_db_value(&status_value).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, Box::new(error))
    })?;

    Ok(Job {
        id: row.get(0)?,
        company: row.get(1)?,
        title: row.get(2)?,
        status,
        applied_on: row.get(4)?,
        comments: row.get(5)?,
    })
}
