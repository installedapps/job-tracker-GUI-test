use super::JobRepository;
use crate::{Job, JobTrackerError};
use rusqlite::{params, Connection};
use std::path::Path;

pub struct SqliteJobRepository {
    connection: Connection,
}

impl SqliteJobRepository {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, JobTrackerError> {
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }
        let repository = Self {
            connection: Connection::open(path)?,
        };
        super::schema::initialize(&repository.connection)?;
        Ok(repository)
    }
}

impl JobRepository for SqliteJobRepository {
    fn add_job(&self, job: &Job) -> Result<(), JobTrackerError> {
        self.connection.execute(
            "INSERT INTO jobs (company, title, status, applied_on, comments)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                job.company,
                job.title,
                job.status.as_db_value(),
                job.applied_on,
                job.comments
            ],
        )?;
        Ok(())
    }

    fn update_job(&self, job: &Job) -> Result<(), JobTrackerError> {
        let id = job.id.ok_or(JobTrackerError::MissingJobId)?;
        self.connection.execute(
            "UPDATE jobs SET company = ?1, title = ?2, status = ?3,
             applied_on = ?4, comments = ?5 WHERE id = ?6",
            params![
                job.company,
                job.title,
                job.status.as_db_value(),
                job.applied_on,
                job.comments,
                id
            ],
        )?;
        Ok(())
    }

    fn delete_job(&self, id: i64) -> Result<(), JobTrackerError> {
        self.connection
            .execute("DELETE FROM jobs WHERE id = ?1", params![id])?;
        Ok(())
    }

    fn load_jobs(&self) -> Result<Vec<Job>, JobTrackerError> {
        let mut statement = self.connection.prepare(
            "SELECT id, company, title, status, applied_on, comments
             FROM jobs ORDER BY id DESC",
        )?;
        let rows = statement.query_map([], super::mapper::row_to_job)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }
}
