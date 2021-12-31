use super::{
    mailbox::MailboxUI,
    report::{Report, ReportType},
    tabline::Tabline,
};
use crate::{terminal::Terminal, ui::prompt::Prompt};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame as TuiFrame,
};

pub type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stdout>>;

pub struct UI {
    mailbox: MailboxUI,
    tabline: Tabline,
    prompt_area: Rect,
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
            mailbox: MailboxUI::new(h_chunks[0]),
            tabline: Tabline::new(v_chunks[0]),
            prompt_area: prompt_chunks[1],
            report: None,
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        f.render_widget(&self.mailbox, self.mailbox.area());
        f.render_widget(&self.tabline, self.tabline.area());

        // let titles = self
        //     .titles
        //     .iter()
        //     .map(|s| Spans::from(s.as_str()))
        //     .collect();
        // let tabs = Tabs::new(titles);
        // let tabs_area = tabs_block.inner(self.tabline_buffer.area);
        // f.render_widget(tabs_block, self.tabline_buffer.area);

        if let Some(ref report) = self.report {
            f.render_widget(report, self.prompt_area)
        }
    }

    pub fn prompt(&mut self, msg: &str) -> Prompt {
        self.report = None;
        Prompt::new(msg, self.prompt_area)
    }

    pub fn new_tab(&mut self, s: String) {
        self.tabline.push_title(s)
    }

    pub fn set_mailbox_names(&mut self, mailboxes: Vec<String>) {
        self.mailbox.set_names(mailboxes);
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
