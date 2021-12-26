use crate::{terminal::Terminal, ui::UI, utils::Event};
use crossterm::event::{
    self,
    Event::{self as CEvent, Key},
    KeyCode, KeyEvent,
};
use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::Widget,
};

use super::Frame;

pub struct Prompt<'msg> {
    msg: &'msg str,
    area: Rect,
    pub response: String,
}

impl<'msg> Prompt<'msg> {
    pub fn new(msg: &'msg str, term: &Terminal) -> Self {
        let rect = term.size().unwrap();
        let prompt_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(rect);
        Self {
            msg,
            area: prompt_chunks[1],
            response: String::new(),
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        f.render_widget(self, self.area);
    }

    pub fn handle_event(&mut self) -> Event {
        if let Ok(e) = event::read() {
            match e {
                Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => {
                    self.response.clear();
                    Event::Break
                }
                Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => Event::Break,
                Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) => {
                    self.response.push(c);
                    Event::None
                }
                Key(KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }) => {
                    let _ = self.response.pop();
                    Event::None
                }
                _ => Event::None,
            }
        } else {
            Event::None
        }
    }
}

impl Widget for &Prompt<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.x, area.y, self.msg, Style::default());
        buf.set_string(
            area.x + self.msg.len() as u16,
            area.y,
            &self.response,
            Style::default(),
        );
    }
}
