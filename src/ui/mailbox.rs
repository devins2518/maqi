use log::info;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Widget},
};

pub struct MailboxUI {
    buf: Buffer,
    mailbox_names: Vec<String>,
}

impl MailboxUI {
    pub fn new(area: Rect) -> Self {
        let mut buf = Buffer::empty(area);
        Block::default()
            .title("Mailboxes")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .render(buf.area, &mut buf);
        Self {
            buf,
            mailbox_names: Vec::new(),
        }
    }

    pub fn area(&self) -> Rect {
        self.buf.area
    }

    pub fn set_names(&mut self, mailboxes: Vec<String>) {
        self.mailbox_names = mailboxes;
        let items: Vec<ListItem> = self
            .mailbox_names
            .iter()
            .map(|x| ListItem::new(x.as_str()))
            .collect();
        List::new(items).render(self.area(), &mut self.buf);
    }
}

impl Widget for &MailboxUI {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        buf.merge(&self.buf)
    }
}
