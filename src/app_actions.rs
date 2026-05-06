use crate::{app, app_defaults};
use job_tracker::{CalendarState, JobForm, JobRepository};

impl app::JobTracker {
    pub(crate) fn application_date(&self) -> String {
        self.form
            .applied_on
            .clone()
            .unwrap_or_else(|| self.calendar.selected_date())
    }

    pub(crate) fn set_date(&mut self, date: String) {
        if let Ok(calendar) = CalendarState::from_date(&date) {
            self.calendar = calendar;
        }
        self.form.applied_on = Some(date);
    }

    pub(crate) fn save_job(&mut self) {
        match self.form.to_job() {
            Ok(job) => self.persist_job(job),
            Err(error) => self.notice = Some(error.to_string()),
        }
    }

    pub(crate) fn edit_job(&mut self, id: i64) {
        if let Some(job) = self.jobs.iter().find(|job| job.id == Some(id)) {
            self.form = JobForm::from_job(job);
            if let Ok(calendar) = CalendarState::from_date(&job.applied_on) {
                self.calendar = calendar;
            }
            self.notice = Some(format!("Editing {}.", job.company));
        }
    }

    pub(crate) fn delete_job(&mut self, id: i64) {
        if let Some(repository) = &self.repository {
            self.reload_after(repository.delete_job(id), "Job deleted.");
        } else {
            self.notice = Some("Database is not available.".into());
        }
    }

    pub(crate) fn clear_form(&mut self) {
        self.form.clear();
        self.calendar = app_defaults::initial_calendar();
        self.calendar_open = false;
        self.notice = None;
    }

    pub(crate) fn select_day(&mut self, day: u32) {
        if self.calendar.select_day(day).is_ok() {
            self.form.applied_on = Some(self.calendar.selected_date());
            self.calendar_open = false;
        }
    }

    fn persist_job(&mut self, job: job_tracker::Job) {
        if let Some(repository) = &self.repository {
            let message = if job.id.is_some() {
                "Job updated."
            } else {
                "Job saved."
            };
            let result = if job.id.is_some() {
                repository.update_job(&job)
            } else {
                repository.add_job(&job)
            };
            self.reload_after(result, message);
        } else {
            self.notice = Some("Database is not available.".into());
        }
    }

    fn reload_after(&mut self, result: Result<(), job_tracker::JobTrackerError>, message: &str) {
        match result.and_then(|_| self.repository.as_ref().unwrap().load_jobs()) {
            Ok(jobs) => {
                self.jobs = jobs;
                self.clear_form();
                self.notice = Some(message.to_string());
            }
            Err(error) => self.notice = Some(error.to_string()),
        }
    }
}
