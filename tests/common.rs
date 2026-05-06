#![allow(dead_code)]

use job_tracker::{Job, Status};

pub fn sample_job(company: &str, title: &str, status: Status) -> Job {
    Job::new(company, title, status, "2026-05-04")
}
