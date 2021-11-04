use crate::application;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    text::Spans,
    widgets::{Paragraph, Tabs},
    Frame,
};

pub fn draw(frame: &mut Frame<CrosstermBackend<Stdout>>, app: &mut application::Application) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(frame.size());
    let titles = app.spans.iter().map(|s| Spans::from(s.as_str())).collect();
    let tabs = Tabs::new(titles);
    frame.render_widget(Paragraph::new("hi"), chunks[0]);
    frame.render_widget(tabs, chunks[1]);
}
