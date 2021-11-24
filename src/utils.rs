pub enum Builtins {
    Gmail,
    ICloud,
    Outlook,
    Yahoo,
    Custom(String, String),
}

impl Builtins {
    pub fn get_addresses(self) -> (/* IMAP */ String, /* SMTP */ String) {
        match self {
            Self::Custom(imap, smtp) => (imap, smtp),
            // TODO
            _ => (String::new(), String::new()),
        }
    }
}
