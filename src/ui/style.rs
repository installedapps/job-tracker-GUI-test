use super::colors;
use iced::widget::container;
use iced::{Border, Color, Shadow, Theme, Vector};
use job_tracker::Status;

const PANEL: Color = Color::from_rgb(0.04, 0.09, 0.18);
const CARD: Color = Color::from_rgb(0.06, 0.12, 0.23);

pub fn hero(_: &Theme) -> container::Style {
    base_panel(Color::from_rgb8(9, 24, 49), Color::from_rgb8(59, 130, 246))
}

pub fn panel(_: &Theme) -> container::Style {
    base_panel(PANEL, Color::from_rgb8(37, 99, 235))
}

pub fn accent_panel(_: &Theme) -> container::Style {
    base_panel(CARD, Color::from_rgb8(30, 64, 175))
}

pub fn calendar_panel(_: &Theme) -> container::Style {
    base_panel(Color::from_rgb8(8, 18, 35), Color::from_rgb8(59, 130, 246))
}

pub fn calendar_empty_cell(_: &Theme) -> container::Style {
    container::Style::default().background(Color::from_rgba8(15, 23, 42, 0.35))
}

pub fn calendar_label(_: &Theme) -> container::Style {
    container::Style::default().color(Color::from_rgb8(147, 197, 253))
}

pub fn status(status: Status) -> container::Style {
    let colours = status.colours();
    container::Style::default()
        .color(hex_color(colours.text))
        .background(hex_color(colours.background))
        .border(border(hex_color(colours.border)))
}

pub fn heatmap_cell(count: usize, max: usize) -> container::Style {
    let intensity = if max == 0 {
        0.0
    } else {
        count as f32 / max as f32
    };
    let blue = (50.0 + intensity * 160.0) as u8;
    container::Style::default()
        .color(colors::TEXT)
        .background(Color::from_rgb8(8, 34, blue))
        .border(
            Border::default()
                .rounded(10)
                .width(1)
                .color(Color::from_rgb8(37, 99, 235)),
        )
}

fn base_panel(background: Color, border_color: Color) -> container::Style {
    container::Style::default()
        .color(colors::TEXT)
        .background(background)
        .border(border(border_color))
        .shadow(Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.28),
            offset: Vector::new(0.0, 10.0),
            blur_radius: 24.0,
        })
}

fn border(color: Color) -> Border {
    Border::default().rounded(16).width(1).color(color)
}

fn hex_color(value: &str) -> Color {
    colors::hex(value)
}
