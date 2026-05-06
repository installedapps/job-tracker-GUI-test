use crate::{app, message::Message, paths};
use iced::Task;
use job_tracker::{SankeyGraph, Stats};
use std::path::PathBuf;

impl app::JobTracker {
    pub(crate) fn export_sankey(&self) -> Task<Message> {
        let path = paths::sankey_export_path();
        let svg = SankeyGraph::from_stats(&Stats::from_jobs(&self.jobs)).to_svg();
        Task::perform(write_svg(path, svg), Message::SankeyExported)
    }

    pub(crate) fn sankey_exported(&mut self, result: Result<PathBuf, String>) {
        self.notice = match result {
            Ok(path) => Some(format!("Sankey exported to {}", path.display())),
            Err(error) => Some(format!("Could not export Sankey: {error}")),
        };
    }
}

async fn write_svg(path: PathBuf, svg: String) -> Result<PathBuf, String> {
    std::fs::write(&path, svg)
        .map(|_| path)
        .map_err(|error| error.to_string())
}
