mod mapper;
mod schema;
mod sqlite;

use crate::{Job, JobTrackerError};

pub use sqlite::SqliteJobRepository;

pub trait JobRepository {
    fn add_job(&self, job: &Job) -> Result<(), JobTrackerError>;
    fn update_job(&self, job: &Job) -> Result<(), JobTrackerError>;
    fn delete_job(&self, id: i64) -> Result<(), JobTrackerError>;
    fn load_jobs(&self) -> Result<Vec<Job>, JobTrackerError>;
}
