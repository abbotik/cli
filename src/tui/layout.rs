use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct FrameLayout {
    pub top_shortcuts: Rect,
    pub header: Rect,
    pub main_sidebar: Rect,
    pub main_content: Rect,
    pub footer: Rect,
    pub composer: Rect,
}

pub fn split(area: Rect) -> FrameLayout {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Min(8),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .split(area);

    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(38), Constraint::Min(40)])
        .split(vertical[2]);

    FrameLayout {
        top_shortcuts: vertical[0],
        header: vertical[1],
        main_sidebar: main[0],
        main_content: main[1],
        footer: vertical[3],
        composer: vertical[4],
    }
}
