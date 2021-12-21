use std::io::{self, Stdout};
use tui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Spans,
    widgets::{Block, BorderType, Borders, Tabs},
    Frame as TuiFrame,
};

use crate::{terminal::Terminal, ui::prompt::Prompt};

use super::report::{Report, ReportType};

pub type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stdout>>;

pub struct UI {
    mailbox_buffer: Buffer,
    tabline_buffer: Buffer,
    prompt_buffer: Buffer,
    main_buffer: Buffer,
    pub spans: Vec<String>,
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
            mailbox_buffer: Buffer::empty(h_chunks[0]),
            tabline_buffer: Buffer::empty(v_chunks[0]),
            prompt_buffer: Buffer::empty(prompt_chunks[1]),
            main_buffer: Buffer::empty(v_chunks[1]),
            spans: Vec::new(),
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        let mb_block = Block::default()
            .title("Mailboxes")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        frame.render_widget(mb_block, self.mailbox_buffer.area);

        let titles = self.spans.iter().map(|s| Spans::from(s.as_str())).collect();
        let tabs = Tabs::new(titles);
        let tabs_block = Block::default()
            // TODO: Remove
            .title("─Tabs")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let tabs_area = tabs_block.inner(self.tabline_buffer.area);
        frame.render_widget(tabs_block, self.tabline_buffer.area);
        frame.render_widget(tabs, tabs_area);
    }

    pub fn prompt(&self, term: &mut Terminal, msg: &str) -> String {
        Prompt::new(msg, &self).run(term, self.prompt_buffer.area)
    }

    pub fn info(&self, term: &mut Terminal, msg: &str) -> io::Result<()> {
        Report::new(ReportType::Info, msg, &self).show(term, self.prompt_buffer.area)?;
        Ok(())
    }

    pub fn warning(&self, term: &mut Terminal, msg: &str) -> io::Result<()> {
        Report::new(ReportType::Warning, msg, &self).show(term, self.prompt_buffer.area)?;
        Ok(())
    }
    pub fn error(&self, term: &mut Terminal, msg: &str) -> io::Result<()> {
        Report::new(ReportType::Error, msg, &self).show(term, self.prompt_buffer.area)?;
        Ok(())
    }
}
