use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::Spans,
    widgets::{Block, BorderType, Borders, Tabs, Widget},
};

pub struct Tabline {
    buf: Buffer,
    titles: Vec<String>,
}

impl Tabline {
    pub fn new(area: Rect) -> Self {
        let mut buf = Buffer::empty(area);
        Block::default()
            // TODO: Remove
            .title("─Tabs")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .render(buf.area, &mut buf);
        Self {
            buf,
            titles: Vec::new(),
        }
    }

    pub fn area(&self) -> Rect {
        self.buf.area
    }

    pub fn push_title(&mut self, s: String) {
        self.titles.push(s);
        let titles = self
            .titles
            .iter()
            .map(|x| Spans::from(x.as_str()))
            .collect();
        Tabs::new(titles)
            .block(
                Block::default()
                    // TODO: Remove
                    .title("─Tabs")
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .render(self.buf.area, &mut self.buf);
    }
}

impl Widget for &Tabline {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        buf.merge(&self.buf)
    }
}
