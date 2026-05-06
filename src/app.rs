use crate::{app_defaults::initial_calendar, message::Message, paths, ui};
use iced::{time, Element, Subscription, Task, Theme};
use job_tracker::{
    ActivePanel, AnimationState, CalendarState, Job, JobForm, JobRepository, SqliteJobRepository,
};
use std::time::Duration;

pub fn run() -> iced::Result {
    iced::application(JobTracker::new, JobTracker::update, JobTracker::view)
        .title("Job Tracker")
        .theme(app_theme)
        .subscription(JobTracker::subscription)
        .run()
}

fn app_theme(_: &JobTracker) -> Theme {
    Theme::Dark
}

pub struct JobTracker {
    pub repository: Option<SqliteJobRepository>,
    pub jobs: Vec<Job>,
    pub form: JobForm,
    pub notice: Option<String>,
    pub active_panel: ActivePanel,
    pub animation: AnimationState,
    pub calendar: CalendarState,
    pub calendar_open: bool,
}

impl JobTracker {
    fn new() -> Self {
        match SqliteJobRepository::open(paths::database_path()) {
            Ok(repository) => Self::with_repository(repository),
            Err(error) => Self::empty(Some(error.to_string()), None),
        }
    }

    fn with_repository(repository: SqliteJobRepository) -> Self {
        let (jobs, notice) = match repository.load_jobs() {
            Ok(jobs) => (jobs, None),
            Err(error) => (Vec::new(), Some(error.to_string())),
        };
        Self::empty(notice, Some(repository)).with_jobs(jobs)
    }

    fn empty(notice: Option<String>, repository: Option<SqliteJobRepository>) -> Self {
        Self {
            repository,
            jobs: Vec::new(),
            form: JobForm::default(),
            notice,
            active_panel: ActivePanel::Jobs,
            animation: AnimationState::default(),
            calendar: initial_calendar(),
            calendar_open: false,
        }
    }

    fn with_jobs(mut self, jobs: Vec<Job>) -> Self {
        self.jobs = jobs;
        self
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CompanyChanged(v) => self.form.company = v,
            Message::TitleChanged(v) => self.form.title = v,
            Message::CommentsChanged(v) => self.form.comments = v,
            Message::StatusChanged(v) => self.form.status = v,
            Message::DateChanged(v) => self.set_date(v),
            Message::SaveJob => self.save_job(),
            Message::EditJob(id) => self.edit_job(id),
            Message::DeleteJob(id) => self.delete_job(id),
            Message::CancelEdit => self.clear_form(),
            Message::ShowPanel(panel) => self.active_panel = self.active_panel.switch_to(panel),
            Message::ExportSankey => return self.export_sankey(),
            Message::SankeyExported(result) => self.sankey_exported(result),
            Message::ToggleCalendar => self.calendar_open = !self.calendar_open,
            Message::PreviousMonth => self.calendar.previous_month(),
            Message::NextMonth => self.calendar.next_month(),
            Message::SelectDay(day) => self.select_day(day),
            Message::Tick if self.active_panel.shows_stats() => self.animation.advance(),
            Message::Tick => {}
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.active_panel.shows_stats() && !self.jobs.is_empty() {
            time::every(Duration::from_millis(320)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<'_, Message> {
        ui::view(self)
    }
}
