use super::{buttons, form, style};
use crate::{app::JobTracker, message::Message};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Element, Fill, Length};
use job_tracker::{Job, Status};

pub fn panel(app: &JobTracker) -> Element<'_, Message> {
    super::panel(
        column![
            form::panel(app),
            row![
                text("Applications").size(24),
                Space::new().width(Fill),
                text(format!("{} tracked", app.jobs.len())).size(14)
            ]
            .align_y(Alignment::Center),
            if app.jobs.is_empty() {
                column![empty_state()].spacing(8)
            } else {
                app.jobs
                    .iter()
                    .fold(column![].spacing(8), |list, job| list.push(row_view(job)))
            }
        ]
        .spacing(16),
    )
}

fn row_view<'a>(job: &'a Job) -> Element<'a, Message> {
    let id = job.id;
    container(
        row![
            column![
                text(&job.company).size(18),
                text(format!("{} - {}", job.title, job.applied_on)).size(13),
                comment_text(job),
            ]
            .spacing(4)
            .width(Fill),
            status_badge(job.status),
            action("Edit", id.map(Message::EditJob), buttons::ghost),
            action("Delete", id.map(Message::DeleteJob), buttons::danger),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
    )
    .padding(12)
    .style(style::accent_panel)
    .into()
}

fn comment_text<'a>(job: &'a Job) -> iced::widget::Text<'a> {
    if job.comments.is_empty() {
        text("").size(1)
    } else {
        text(&job.comments).size(13)
    }
}

fn action<'a>(
    label: &'static str,
    message: Option<Message>,
    style_fn: fn(&iced::Theme, button::Status) -> button::Style,
) -> Element<'a, Message> {
    button(label)
        .on_press_maybe(message)
        .padding([7, 12])
        .style(style_fn)
        .into()
}

fn status_badge<'a>(status: Status) -> Element<'a, Message> {
    container(text(status.to_string()).size(14))
        .padding([7, 12])
        .style(move |_| style::status(status))
        .into()
}

fn empty_state<'a>() -> Element<'a, Message> {
    container(
        row![
            Space::new().width(Length::FillPortion(1)),
            text("No applications yet. Add your first job above.").size(16),
            Space::new().width(Length::FillPortion(1)),
        ]
        .align_y(Alignment::Center),
    )
    .padding(24)
    .style(style::accent_panel)
    .into()
}
