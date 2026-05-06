use job_tracker::CalendarState;

pub fn initial_calendar() -> CalendarState {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    CalendarState::from_date(&today).unwrap_or_else(|_| fallback_calendar())
}

fn fallback_calendar() -> CalendarState {
    CalendarState::from_date("2026-05-05").unwrap()
}
