use super::{buttons, month, style};
use crate::message::Message;
use iced::widget::{button, column, container, row, rule, text};
use iced::{Alignment, Element, Fill, Length};
use job_tracker::CalendarState;

pub fn dropdown<'a>(calendar: CalendarState) -> Element<'a, Message> {
    let days = (0..6).fold(column![].spacing(7), |weeks, week| {
        weeks.push(week_row(calendar, week))
    });

    container(column![header(calendar), rule::horizontal(1), labels(), days,].spacing(10))
        .padding(16)
        .width(Fill)
        .style(style::calendar_panel)
        .into()
}

fn header<'a>(calendar: CalendarState) -> Element<'a, Message> {
    row![
        nav("<", Message::PreviousMonth),
        column![
            text(format!(
                "{} {}",
                month::name(calendar.month()),
                calendar.year()
            ))
            .size(20),
            text(format!("Selected {}", calendar.selected_date())).size(12),
        ]
        .spacing(2)
        .width(Fill)
        .align_x(Alignment::Center),
        nav(">", Message::NextMonth),
    ]
    .spacing(10)
    .align_y(Alignment::Center)
    .into()
}

fn nav<'a>(label: &'static str, message: Message) -> Element<'a, Message> {
    button(label)
        .on_press(message)
        .padding([8, 14])
        .style(buttons::calendar_nav)
        .into()
}

fn labels<'a>() -> Element<'a, Message> {
    ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
        .into_iter()
        .fold(row![].spacing(6), |row, label| row.push(label_cell(label)))
        .into()
}

fn label_cell<'a>(label: &'static str) -> Element<'a, Message> {
    container(text(label).size(12))
        .width(Length::FillPortion(1))
        .center_x(Fill)
        .style(style::calendar_label)
        .into()
}

fn week_row<'a>(calendar: CalendarState, week: u32) -> Element<'a, Message> {
    let leading = calendar.leading_blank_days();
    let end = leading + calendar.days_in_month();
    (0..7)
        .fold(row![].spacing(7), |row, weekday| {
            let cell = week * 7 + weekday;
            if cell < leading || cell >= end {
                row.push(empty_cell())
            } else {
                row.push(day_cell(calendar, cell - leading + 1))
            }
        })
        .into()
}

fn day_cell<'a>(calendar: CalendarState, day: u32) -> Element<'a, Message> {
    let selected = day == calendar.day();
    button(container(text(day.to_string()).size(14)).center_x(Fill))
        .on_press(Message::SelectDay(day))
        .padding([9, 0])
        .width(Length::FillPortion(1))
        .style(move |theme, status| buttons::calendar_day(theme, status, selected))
        .into()
}

fn empty_cell<'a>() -> Element<'a, Message> {
    container(text(""))
        .padding([9, 0])
        .width(Length::FillPortion(1))
        .style(style::calendar_empty_cell)
        .into()
}
