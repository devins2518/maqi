use std::io;

use tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use crate::terminal::Terminal;

use super::UI;

pub enum ReportType {
    Info,
    Warning,
    Error,
}

pub struct Report<'msg, 'ui> {
    r_type: ReportType,
    msg: &'msg str,
    ui: &'ui UI,
}

impl<'msg, 'ui> Report<'msg, 'ui> {
    pub fn new(r_type: ReportType, msg: &'msg str, ui: &'ui UI) -> Self {
        Self { r_type, msg, ui }
    }

    pub fn show(self, term: &mut Terminal, area: Rect) -> io::Result<()> {
        term.draw(|f| {
            self.ui.draw(f);
            f.render_widget(self, area);
        })?;
        Ok(())
    }
}

impl Widget for Report<'_, '_> {
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let color = match self.r_type {
            ReportType::Info => Color::Blue,
            ReportType::Warning => Color::Yellow,
            ReportType::Error => Color::Red,
        };
        buf.set_string(area.x, area.y, self.msg, Style::default().fg(color));
    }
}
