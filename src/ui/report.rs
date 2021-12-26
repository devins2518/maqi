use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

#[derive(Debug)]
pub enum ReportType {
    Info,
    Warning,
    Error,
}

pub struct Report {
    r_type: ReportType,
    msg: String,
}

impl Report {
    pub fn new(r_type: ReportType, msg: &str) -> Self {
        Self {
            r_type,
            msg: String::from(msg),
        }
    }
}

impl Widget for &Report {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let color = match self.r_type {
            ReportType::Info => Color::Blue,
            ReportType::Warning => Color::Yellow,
            ReportType::Error => Color::Red,
        };
        buf.set_string(area.x, area.y, &self.msg, Style::default().fg(color));
    }
}
