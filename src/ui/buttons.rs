use super::colors;
use iced::widget::button;
use iced::{Border, Color, Shadow, Theme, Vector};

pub fn blue(theme: &Theme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Hovered => Color::from_rgb8(59, 130, 246),
        button::Status::Pressed => Color::from_rgb8(29, 78, 216),
        _ => colors::BLUE,
    };
    base(button::primary(theme, status), background, Color::WHITE)
}

pub fn ghost(theme: &Theme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Hovered => Color::from_rgb8(23, 37, 84),
        button::Status::Pressed => Color::from_rgb8(30, 64, 175),
        _ => Color::from_rgb8(15, 23, 42),
    };
    base(button::secondary(theme, status), background, colors::TEXT)
}

pub fn danger(theme: &Theme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Hovered => Color::from_rgb8(190, 18, 60),
        button::Status::Pressed => Color::from_rgb8(159, 18, 57),
        _ => Color::from_rgb8(127, 29, 29),
    };
    base(button::danger(theme, status), background, Color::WHITE)
}

pub fn calendar_nav(theme: &Theme, status: button::Status) -> button::Style {
    ghost(theme, status)
}

pub fn calendar_day(theme: &Theme, status: button::Status, selected: bool) -> button::Style {
    if selected {
        blue(theme, status)
    } else {
        ghost(theme, status)
    }
}

fn base(mut style: button::Style, background: Color, text: Color) -> button::Style {
    style = style.with_background(background);
    style.text_color = text;
    style.border = Border::default()
        .rounded(999)
        .width(1)
        .color(Color::from_rgba(0.58, 0.72, 1.0, 0.22));
    style.shadow = Shadow {
        color: Color::from_rgba(0.0, 0.0, 0.0, 0.22),
        offset: Vector::new(0.0, 4.0),
        blur_radius: 12.0,
    };
    style
}
