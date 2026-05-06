mod common;

use common::sample_job;
use job_tracker::{CalendarState, JobForm, Status};

#[test]
fn form_validation_rejects_missing_company_or_title() {
    let mut form = JobForm {
        title: "Backend Engineer".into(),
        ..JobForm::default()
    };
    assert!(form.to_job().is_err());
    form.company = "Acme".to_string();
    form.title.clear();
    assert!(form.to_job().is_err());
    form.title = "Backend Engineer".to_string();
    assert!(form.to_job().is_ok());
}

#[test]
fn form_validation_rejects_invalid_application_dates() {
    let mut form = valid_form();
    form.applied_on = Some("2026-02-30".to_string());
    assert!(form.to_job().is_err());
}

#[test]
fn form_preserves_application_date_and_comments() {
    let mut form = valid_form();
    form.comments = "Follow up next week.".to_string();
    let job = form.to_job().unwrap();
    assert_eq!(job.applied_on, "2026-05-05");
    assert_eq!(job.comments, "Follow up next week.");
}

#[test]
fn form_can_be_populated_from_an_existing_job_for_editing() {
    let mut job = sample_job("Acme", "Backend Engineer", Status::Applied);
    job.id = Some(42);
    job.comments = "Existing note.".to_string();
    let edited = JobForm::from_job(&job).to_job().unwrap();
    assert_eq!(edited.id, Some(42));
    assert_eq!(edited.applied_on, "2026-05-04");
    assert_eq!(edited.comments, "Existing note.");
}

#[test]
fn calendar_selects_dates_and_handles_leap_years() {
    let mut calendar = CalendarState::from_date("2026-05-05").unwrap();
    calendar.select_day(20).unwrap();
    assert_eq!(calendar.selected_date(), "2026-05-20");
    assert_eq!(
        CalendarState::from_date("2024-02-01")
            .unwrap()
            .days_in_month(),
        29
    );
    assert_eq!(
        CalendarState::from_date("2026-02-01")
            .unwrap()
            .days_in_month(),
        28
    );
}

#[test]
fn calendar_tracks_month_navigation_and_weekday_offsets() {
    let mut calendar = CalendarState::from_date("2026-05-31").unwrap();
    assert_eq!(calendar.leading_blank_days(), 4);
    calendar.next_month();
    assert_eq!(calendar.selected_date(), "2026-06-30");
    assert_eq!(calendar.leading_blank_days(), 0);
    calendar.previous_month();
    assert_eq!(calendar.selected_date(), "2026-05-30");
}

fn valid_form() -> JobForm {
    JobForm {
        company: "Acme".into(),
        title: "Backend Engineer".into(),
        applied_on: Some("2026-05-05".into()),
        ..JobForm::default()
    }
}
