use iced::Color;

pub const TEXT: Color = Color::from_rgb(0.89, 0.93, 0.98);
pub const BLUE: Color = Color::from_rgb(0.23, 0.51, 0.96);

pub fn hex(value: &str) -> Color {
    let value = value.trim_start_matches('#');
    let red = u8::from_str_radix(&value[0..2], 16).unwrap_or(255);
    let green = u8::from_str_radix(&value[2..4], 16).unwrap_or(255);
    let blue = u8::from_str_radix(&value[4..6], 16).unwrap_or(255);
    Color::from_rgb8(red, green, blue)
}
