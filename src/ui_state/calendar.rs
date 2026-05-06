#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarState {
    year: i32,
    month: u32,
    day: u32,
}

impl CalendarState {
    pub fn from_date(date: &str) -> Result<Self, chrono::ParseError> {
        use chrono::{Datelike, NaiveDate};
        let date = NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
        Ok(Self {
            year: date.year(),
            month: date.month(),
            day: date.day(),
        })
    }

    pub fn selected_date(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    pub const fn year(self) -> i32 {
        self.year
    }

    pub const fn month(self) -> u32 {
        self.month
    }

    pub const fn day(self) -> u32 {
        self.day
    }

    pub fn days_in_month(self) -> u32 {
        days_in_month(self.year, self.month)
    }

    pub fn leading_blank_days(self) -> u32 {
        use chrono::{Datelike, NaiveDate};
        NaiveDate::from_ymd_opt(self.year, self.month, 1)
            .map(|date| date.weekday().num_days_from_monday())
            .unwrap_or(0)
    }

    pub fn select_day(&mut self, day: u32) -> Result<(), &'static str> {
        if day == 0 || day > self.days_in_month() {
            return Err("day is outside the visible month");
        }
        self.day = day;
        Ok(())
    }

    pub fn previous_month(&mut self) {
        if self.month == 1 {
            self.month = 12;
            self.year -= 1;
        } else {
            self.month -= 1;
        }
        self.clamp_day();
    }

    pub fn next_month(&mut self) {
        if self.month == 12 {
            self.month = 1;
            self.year += 1;
        } else {
            self.month += 1;
        }
        self.clamp_day();
    }

    fn clamp_day(&mut self) {
        self.day = self.day.min(self.days_in_month());
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 30,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
