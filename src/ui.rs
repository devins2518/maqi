use crate::application;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Tabs},
    Frame,
};

pub fn draw(frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(frame.size());
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(h_chunks[1]);
    let mb_block = Block::default()
        .title("Mailboxes")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    frame.render_widget(mb_block, h_chunks[0]);

    let tabs_block = Block::default()
        // TODO: Remove
        .title("─Tabs")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    frame.render_widget(tabs_block, v_chunks[0]);
}
