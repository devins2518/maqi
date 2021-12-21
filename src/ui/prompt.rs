use crossterm::event::{self, Event::Key, KeyCode, KeyEvent};
use tui::{
    backend::Backend,
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::Widget,
};

use crate::{terminal::Terminal, ui::UI};

pub(super) struct Prompt<'msg, 'ui> {
    msg: &'msg str,
    response: String,
    ui: &'ui UI,
}

impl<'msg, 'ui> Prompt<'msg, 'ui> {
    pub fn new(msg: &'msg str, ui: &'ui UI) -> Self {
        Prompt {
            msg,
            response: String::new(),
            ui,
        }
    }

    pub fn run(mut self, term: &mut Terminal, area: Rect) -> String {
        term.set_cursor(area.x, area.y).unwrap();
        term.show_cursor().unwrap();
        loop {
            term.draw(|f| {
                self.ui.draw(f);
                f.render_widget(&self, area);
            })
            .unwrap();

            if let Ok(e) = event::read() {
                match e {
                    Key(KeyEvent {
                        code: KeyCode::Esc, ..
                    }) => {
                        self.response = String::new();
                        break;
                    }
                    Key(KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    }) => break,
                    Key(KeyEvent {
                        code: KeyCode::Char(c),
                        ..
                    }) => self.response.push(c),
                    _ => (),
                };
            }
        }
        term.hide_cursor().unwrap();

        self.response
    }
}

impl<'msg, 'ui> Widget for &Prompt<'msg, 'ui> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(
            area.x,
            area.y,
            format!("{}{}", self.msg, self.response),
            Style::default(),
        );
    }
}
