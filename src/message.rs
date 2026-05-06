use job_tracker::{ActivePanel, Status};
use std::path::PathBuf;
#[derive(Debug, Clone)]
pub enum Message {
    CompanyChanged(String),
    TitleChanged(String),
    DateChanged(String),
    CommentsChanged(String),
    StatusChanged(Status),
    SaveJob,
    EditJob(i64),
    DeleteJob(i64),
    CancelEdit,
    ShowPanel(ActivePanel),
    ExportSankey,
    SankeyExported(Result<PathBuf, String>),
    ToggleCalendar,
    PreviousMonth,
    NextMonth,
    SelectDay(u32),
    Tick,
}
