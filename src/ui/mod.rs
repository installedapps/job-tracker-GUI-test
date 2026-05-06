pub mod buttons;
mod calendar;
pub mod colors;
mod form;
mod heatmap;
mod jobs;
mod month;
mod stats;
pub mod style;

use crate::{app::JobTracker, message::Message};
use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Alignment, Element, Fill, Theme};
use job_tracker::ActivePanel;

pub fn view(app: &JobTracker) -> Element<'_, Message> {
    let active_panel = match app.active_panel {
        ActivePanel::Jobs => jobs::panel(app),
        ActivePanel::Stats => stats::panel(&app.jobs, app.animation),
    };

    let content = column![
        hero(),
        row![
            panel_button("Applications", ActivePanel::Jobs, app.active_panel),
            panel_button("Stats", ActivePanel::Stats, app.active_panel),
            Space::new().width(Fill),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        active_panel,
    ]
    .spacing(18)
    .padding(24)
    .width(Fill);

    container(scrollable(content))
        .width(Fill)
        .height(Fill)
        .center_x(Fill)
        .style(|_| container::background(iced::Color::from_rgb8(3, 9, 20)))
        .into()
}

fn hero<'a>() -> Element<'a, Message> {
    container(
        column![
            text("Job Tracker").size(40),
            text("A focused pipeline for applications, notes, dates, and outcomes.").size(16),
        ]
        .spacing(6),
    )
    .padding(22)
    .width(Fill)
    .style(style::hero)
    .into()
}

pub fn panel<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .padding(20)
        .width(Fill)
        .style(style::panel)
        .into()
}

fn panel_button<'a>(
    label: &'static str,
    panel: ActivePanel,
    active: ActivePanel,
) -> Element<'a, Message> {
    use iced::widget::button;
    button(label)
        .on_press(Message::ShowPanel(panel))
        .padding([9, 18])
        .style(move |theme: &Theme, status| {
            if panel == active {
                buttons::blue(theme, status)
            } else {
                buttons::ghost(theme, status)
            }
        })
        .into()
}
