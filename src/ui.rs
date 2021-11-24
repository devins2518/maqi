use crate::application;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    text::Spans,
    widgets::{Block, BorderType, Borders, Tabs},
    Frame,
};

pub fn draw(frame: &mut Frame<CrosstermBackend<Stdout>>, app: &mut application::Application) {
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(frame.size());
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(h_chunks[1]);
    let titles = app.spans.iter().map(|s| Spans::from(s.as_str())).collect();
    let mb_block = Block::default()
        .title("Mailboxes")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let mb_area = mb_block.inner(h_chunks[0]);
    frame.render_widget(mb_block, h_chunks[0]);

    let tabs_block = Block::default()
        // TODO: Remove
        .title("─Tabs")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let tabs = Tabs::new(titles);
    let tabs_area = tabs_block.inner(v_chunks[0]);
    frame.render_widget(tabs_block, v_chunks[0]);
    frame.render_widget(tabs, tabs_area);
}
