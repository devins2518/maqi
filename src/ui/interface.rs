use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Spans,
    widgets::{Block, BorderType, Borders, Tabs},
    Frame as TuiFrame,
};

use crate::ui::prompt::Prompt;

type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stdout>>;

pub struct UI {
    prompt_chunks: Vec<Rect>,
    h_chunks: Vec<Rect>,
    v_chunks: Vec<Rect>,
}

impl UI {
    pub fn new(rect: &Rect) -> Self {
        let prompt_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(*rect);
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(prompt_chunks[0]);
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(h_chunks[1]);
        Self {
            prompt_chunks,
            h_chunks,
            v_chunks,
        }
    }

    pub fn draw(&self, frame: &mut Frame, spans: &[String]) {
        let mb_block = Block::default()
            .title("Mailboxes")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let mb_area = mb_block.inner(self.h_chunks[0]);
        frame.render_widget(mb_block, self.h_chunks[0]);

        let titles = spans.iter().map(|s| Spans::from(s.as_str())).collect();
        let tabs = Tabs::new(titles);
        let tabs_block = Block::default()
            // TODO: Remove
            .title("─Tabs")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let tabs_area = tabs_block.inner(self.v_chunks[0]);
        frame.render_widget(tabs_block, self.v_chunks[0]);
        frame.render_widget(tabs, tabs_area);
    }

    pub fn prompt(&self, frame: &mut Frame, msg: &str) -> String {
        let prompt = Prompt::new(msg);

        unimplemented!()
    }
}
