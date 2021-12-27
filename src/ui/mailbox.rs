use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Block, BorderType, Borders, Widget},
};

pub struct Mailbox {
    buf: Buffer,
}

impl Mailbox {
    pub fn new(area: Rect) -> Self {
        let mut buf = Buffer::empty(area);
        Block::default()
            .title("Mailboxes")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .render(buf.area, &mut buf);
        Self { buf }
    }

    pub fn area(&self) -> Rect {
        self.buf.area
    }
}

impl Widget for &Mailbox {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        buf.merge(&self.buf)
    }
}
