use super::{buttons, colors, heatmap, panel as wrap_panel, style};
use crate::message::Message;
use iced::widget::{button, column, progress_bar, row, rule, text};
use iced::{Alignment, Background, Border, Element, Fill, Length};
use job_tracker::{AnimationState, Job, Stats, Status};

pub fn panel<'a>(jobs: &'a [Job], animation: AnimationState) -> Element<'a, Message> {
    let stats = Stats::from_jobs(jobs);
    wrap_panel(
        column![
            row![
                text("Pipeline Stats").size(24).width(Fill),
                button("Export Sankey SVG")
                    .on_press(Message::ExportSankey)
                    .padding([9, 16])
                    .style(buttons::blue),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            heatmap::panel(jobs),
            cards(&stats),
            rule::horizontal(1),
            text("Status Breakdown").size(20),
            graph_row(&stats, Status::Applied, animation),
            graph_row(&stats, Status::Interviewing, animation),
            graph_row(&stats, Status::Rejected, animation),
            graph_row(&stats, Status::Pending, animation),
        ]
        .spacing(14),
    )
}

fn cards<'a>(stats: &Stats) -> Element<'a, Message> {
    row![
        stat_card("Total", stats.total),
        stat_card("Applied", stats.count_for(Status::Applied)),
        stat_card("Interviewing", stats.count_for(Status::Interviewing)),
        stat_card("Rejected", stats.count_for(Status::Rejected)),
        stat_card("Pending", stats.count_for(Status::Pending)),
    ]
    .spacing(10)
    .into()
}

fn stat_card<'a>(label: &'static str, value: usize) -> Element<'a, Message> {
    iced::widget::container(
        column![text(value).size(30), text(label).size(13)]
            .spacing(4)
            .align_x(Alignment::Center),
    )
    .padding(14)
    .width(Length::FillPortion(1))
    .style(style::accent_panel)
    .into()
}

fn graph_row<'a>(stats: &Stats, status: Status, animation: AnimationState) -> Element<'a, Message> {
    let colours = status.colours();
    row![
        text(status.to_string()).width(120),
        progress_bar(0.0..=1.0, animation.bar_value(stats.percentage_for(status)))
            .length(Fill)
            .girth(18)
            .style(move |_| iced::widget::progress_bar::Style {
                background: Background::Color(iced::Color::from_rgb8(15, 23, 42)),
                bar: Background::Color(colors::hex(colours.border)),
                border: Border::default().rounded(6),
            }),
        text(format!("{}", stats.count_for(status))).width(36),
    ]
    .spacing(10)
    .align_y(Alignment::Center)
    .into()
}
