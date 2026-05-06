mod app;
mod app_actions;
mod app_defaults;
mod app_export;
mod message;
mod paths;
mod ui;

pub fn main() -> iced::Result {
    app::run()
}
