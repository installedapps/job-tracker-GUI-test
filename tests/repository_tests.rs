mod common;

use common::sample_job;
use job_tracker::{JobRepository, SqliteJobRepository, Status};

#[test]
fn sqlite_repository_saves_and_loads_jobs() {
    let repository = repository();
    repository
        .add_job(&sample_job("Acme", "Backend Engineer", Status::Applied))
        .unwrap();
    repository
        .add_job(&sample_job(
            "Globex",
            "Product Manager",
            Status::Interviewing,
        ))
        .unwrap();
    let jobs = repository.load_jobs().unwrap();
    assert_eq!(jobs.len(), 2);
    assert_eq!(jobs[0].company, "Globex");
    assert_eq!(jobs[1].company, "Acme");
}

#[test]
fn sqlite_repository_saves_dates_and_comments() {
    let repository = repository();
    let mut job = sample_job("Acme", "Backend Engineer", Status::Applied);
    job.applied_on = "2026-05-05".to_string();
    job.comments = "Strong match, waiting for recruiter reply.".to_string();
    repository.add_job(&job).unwrap();
    let jobs = repository.load_jobs().unwrap();
    assert_eq!(jobs[0].applied_on, "2026-05-05");
    assert_eq!(
        jobs[0].comments,
        "Strong match, waiting for recruiter reply."
    );
}

#[test]
fn sqlite_repository_updates_existing_jobs() {
    let repository = repository();
    repository
        .add_job(&sample_job("Acme", "Backend Engineer", Status::Applied))
        .unwrap();
    let mut job = repository.load_jobs().unwrap().remove(0);
    job.company = "Acme Labs".to_string();
    job.title = "Platform Engineer".to_string();
    job.status = Status::Interviewing;
    job.comments = "Screen scheduled.".to_string();
    repository.update_job(&job).unwrap();
    let jobs = repository.load_jobs().unwrap();
    assert_eq!(jobs[0].company, "Acme Labs");
    assert_eq!(jobs[0].comments, "Screen scheduled.");
}

#[test]
fn sqlite_repository_deletes_jobs_by_id() {
    let repository = repository();
    repository
        .add_job(&sample_job("Acme", "Backend Engineer", Status::Applied))
        .unwrap();
    repository
        .add_job(&sample_job("Globex", "Product Manager", Status::Pending))
        .unwrap();
    let jobs = repository.load_jobs().unwrap();
    repository.delete_job(jobs[0].id.unwrap()).unwrap();
    let remaining = repository.load_jobs().unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].company, "Acme");
}

fn repository() -> SqliteJobRepository {
    let tempdir = tempfile::tempdir().unwrap();
    let repository = SqliteJobRepository::open(tempdir.path().join("jobs.sqlite3")).unwrap();
    std::mem::forget(tempdir);
    repository
}
