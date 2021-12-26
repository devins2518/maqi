use crate::utils::Event;
use crossterm::event::{self, Event::Key, KeyCode, KeyEvent};
use tui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use super::Frame;

pub struct Prompt {
    len: u16,
    area: Rect,
    pub response: String,
    buf: Buffer,
}

impl Prompt {
    pub fn new(msg: &str, area: Rect) -> Self {
        let mut buf = Buffer::empty(area);
        buf.resize(area);
        buf.set_string(buf.area.x, buf.area.y, msg, Style::default());
        Self {
            len: msg.len() as u16,
            area,
            response: String::new(),
            buf,
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
                    self.buf.set_string(
                        self.buf.area.x + self.len,
                        self.buf.area.y,
                        &self.response,
                        Style::default(),
                    );
                    Event::None
                }
                Key(KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }) => {
                    let _ = self.response.pop();
                    self.buf.set_string(
                        self.buf.area.x + self.len + self.response.len() as u16,
                        self.buf.area.y,
                        " ",
                        Style::default(),
                    );
                    Event::None
                }
                _ => Event::None,
            }
        } else {
            Event::None
        }
    }
}

impl Widget for &Prompt {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.merge(&self.buf);
    }
}
