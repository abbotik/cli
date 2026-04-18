use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Copy)]
pub struct Theme;

impl Theme {
    pub fn bg() -> Color {
        Color::Rgb(12, 14, 18)
    }

    pub fn text() -> Style {
        Style::default()
            .fg(Color::Rgb(230, 232, 235))
            .bg(Self::bg())
    }

    pub fn dim() -> Style {
        Style::default()
            .fg(Color::Rgb(123, 132, 143))
            .bg(Self::bg())
    }

    pub fn accent() -> Style {
        Style::default()
            .fg(Color::Rgb(104, 181, 235))
            .bg(Self::bg())
            .add_modifier(Modifier::BOLD)
    }

    pub fn success() -> Style {
        Style::default().fg(Color::Rgb(88, 184, 114)).bg(Self::bg())
    }

    pub fn warning() -> Style {
        Style::default().fg(Color::Rgb(214, 170, 88)).bg(Self::bg())
    }

    pub fn error() -> Style {
        Style::default().fg(Color::Rgb(222, 97, 97)).bg(Self::bg())
    }

    pub fn selected() -> Style {
        Style::default()
            .fg(Color::Rgb(235, 245, 252))
            .bg(Color::Rgb(28, 48, 66))
            .add_modifier(Modifier::BOLD)
    }

    pub fn chrome() -> Style {
        Style::default()
            .fg(Color::Rgb(151, 160, 169))
            .bg(Self::bg())
    }
}
