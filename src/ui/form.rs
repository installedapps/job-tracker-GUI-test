use super::{buttons, calendar, panel as wrap_panel};
use crate::{app::JobTracker, message::Message};
use iced::widget::{button, column, pick_list, row, text, text_input};
use iced::{Alignment, Element, Fill, Length};
use job_tracker::Status;

pub fn panel_view(app: &JobTracker) -> Element<'_, Message> {
    wrap_panel(
        column![
            header(app.form.is_editing()),
            row![
                text_input("Company", &app.form.company)
                    .on_input(Message::CompanyChanged)
                    .padding(12)
                    .width(Length::FillPortion(2)),
                text_input("Job title", &app.form.title)
                    .on_input(Message::TitleChanged)
                    .padding(12)
                    .width(Length::FillPortion(2)),
                pick_list(Status::all(), Some(app.form.status), Message::StatusChanged)
                    .placeholder("Status")
                    .padding(12)
                    .width(Length::FillPortion(1)),
            ]
            .spacing(10),
            details(app),
            if app.calendar_open {
                calendar::dropdown(app.calendar)
            } else {
                column![].into()
            },
            save_button(app.form.is_editing()),
            app.notice
                .as_deref()
                .map(text)
                .unwrap_or_else(|| text(""))
                .size(14)
        ]
        .spacing(12),
    )
}

pub use panel_view as panel;

fn header<'a>(editing: bool) -> Element<'a, Message> {
    row![
        text(if editing {
            "Edit application"
        } else {
            "Add application"
        })
        .size(24)
        .width(Fill),
        button(if editing { "Cancel" } else { "Clear" })
            .on_press(Message::CancelEdit)
            .padding([8, 14])
            .style(buttons::ghost)
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn details(app: &JobTracker) -> Element<'_, Message> {
    row![
        text_input("Application date", &app.application_date())
            .on_input(Message::DateChanged)
            .padding(12)
            .width(Length::FillPortion(2)),
        button("Calendar")
            .on_press(Message::ToggleCalendar)
            .padding([10, 18])
            .style(buttons::ghost),
        text_input("Comments", &app.form.comments)
            .on_input(Message::CommentsChanged)
            .padding(12)
            .width(Length::FillPortion(3)),
    ]
    .spacing(10)
    .align_y(Alignment::Center)
    .into()
}

fn save_button<'a>(editing: bool) -> Element<'a, Message> {
    button(if editing { "Update job" } else { "Save job" })
        .on_press(Message::SaveJob)
        .padding([10, 18])
        .style(buttons::blue)
        .into()
}
