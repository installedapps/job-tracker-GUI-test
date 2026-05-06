use crate::message::Message;
use iced::widget::{column, container, row, text};
use iced::{Element, Fill, Length};
use job_tracker::{ApplicationHeatmap, Job};

use super::style;

pub fn panel<'a>(jobs: &'a [Job]) -> Element<'a, Message> {
    let heatmap = ApplicationHeatmap::from_jobs(jobs);
    container(
        column![
            text("Application Heatmap").size(22),
            text(format!(
                "{} active days | peak {} applications",
                heatmap.active_days(),
                heatmap.max_count()
            ))
            .size(13),
            cells(&heatmap),
        ]
        .spacing(10),
    )
    .padding(16)
    .width(Fill)
    .style(style::accent_panel)
    .into()
}

fn cells<'a>(heatmap: &ApplicationHeatmap) -> Element<'a, Message> {
    let cells: Vec<_> = heatmap.cells().collect();
    if cells.is_empty() {
        return text("No application dates yet.").size(14).into();
    }

    cells
        .chunks(14)
        .fold(column![].spacing(6), |column, chunk| {
            column.push(chunk.iter().fold(row![].spacing(6), |row, (date, count)| {
                row.push(cell(date, *count, heatmap.max_count()))
            }))
        })
        .into()
}

fn cell<'a>(_date: &str, count: usize, max: usize) -> Element<'a, Message> {
    container(text(count.to_string()).size(13))
        .padding([8, 0])
        .width(Length::FillPortion(1))
        .center_x(Fill)
        .style(move |_| style::heatmap_cell(count, max))
        .into()
}
