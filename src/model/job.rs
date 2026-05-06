use crate::{JobTrackerError, Status};
use chrono::{Local, NaiveDate};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job {
    pub id: Option<i64>,
    pub company: String,
    pub title: String,
    pub status: Status,
    pub applied_on: String,
    pub comments: String,
}

impl Job {
    pub fn new(company: &str, title: &str, status: Status, applied_on: &str) -> Self {
        Self {
            id: None,
            company: company.trim().to_string(),
            title: title.trim().to_string(),
            status,
            applied_on: applied_on.trim().to_string(),
            comments: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JobForm {
    pub id: Option<i64>,
    pub company: String,
    pub title: String,
    pub status: Status,
    pub applied_on: Option<String>,
    pub comments: String,
}

impl Default for JobForm {
    fn default() -> Self {
        Self {
            id: None,
            company: String::new(),
            title: String::new(),
            status: Status::Applied,
            applied_on: None,
            comments: String::new(),
        }
    }
}

impl JobForm {
    pub fn from_job(job: &Job) -> Self {
        Self {
            id: job.id,
            company: job.company.clone(),
            title: job.title.clone(),
            status: job.status,
            applied_on: Some(job.applied_on.clone()),
            comments: job.comments.clone(),
        }
    }

    pub fn to_job(&self) -> Result<Job, JobTrackerError> {
        self.validate()?;
        let applied_on = self
            .applied_on
            .clone()
            .unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());

        Ok(Job {
            id: self.id,
            company: self.company.trim().to_string(),
            title: self.title.trim().to_string(),
            status: self.status,
            applied_on,
            comments: self.comments.trim().to_string(),
        })
    }

    pub fn is_editing(&self) -> bool {
        self.id.is_some()
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    fn validate(&self) -> Result<(), JobTrackerError> {
        if self.company.trim().is_empty() {
            return Err(JobTrackerError::Validation("Company is required.".into()));
        }
        if self.title.trim().is_empty() {
            return Err(JobTrackerError::Validation("Role is required.".into()));
        }
        if let Some(date) = &self.applied_on {
            NaiveDate::parse_from_str(date, "%Y-%m-%d")
                .map_err(|_| JobTrackerError::Validation("Use a valid application date.".into()))?;
        }
        Ok(())
    }
}
