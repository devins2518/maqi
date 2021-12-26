use super::report::{Report, ReportType};
use crate::{terminal::Terminal, ui::prompt::Prompt};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout},
    text::Spans,
    widgets::{Block, BorderType, Borders, Tabs},
    Frame as TuiFrame,
};

pub type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stdout>>;

pub struct UI {
    mailbox_buffer: Buffer,
    tabline_buffer: Buffer,
    prompt_buffer: Buffer,
    main_buffer: Buffer,
    pub titles: Vec<String>,
    report: Option<Report>,
}

impl UI {
    pub fn new(term: &Terminal) -> Self {
        let rect = term.size().unwrap();
        let prompt_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(rect);
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
            titles: Vec::new(),
            report: None,
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        let mb_block = Block::default()
            .title("Mailboxes")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        f.render_widget(mb_block, self.mailbox_buffer.area);

        let titles = self
            .titles
            .iter()
            .map(|s| Spans::from(s.as_str()))
            .collect();
        let tabs = Tabs::new(titles);
        let tabs_block = Block::default()
            // TODO: Remove
            .title("─Tabs")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let tabs_area = tabs_block.inner(self.tabline_buffer.area);
        f.render_widget(tabs_block, self.tabline_buffer.area);

        if let Some(ref report) = self.report {
            f.render_widget(report, self.prompt_buffer.area)
        }
    }

    pub fn prompt(&self, msg: &str) -> Prompt {
        Prompt::new(msg, self.prompt_buffer.area)
    }

    // TODO: These don't expire
    pub fn info(&mut self, msg: &str) {
        self.report = Some(Report::new(ReportType::Info, msg));
    }

    pub fn warning(&mut self, msg: &str) {
        self.report = Some(Report::new(ReportType::Warning, msg));
    }
    pub fn error(&mut self, msg: &str) {
        self.report = Some(Report::new(ReportType::Error, msg));
    }
}
