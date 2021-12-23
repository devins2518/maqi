pub enum Provider<'a> {
    Gmail,
    ICloud,
    Outlook,
    Yahoo,
    Custom(&'a str, &'a str),
}

impl<'a> Provider<'a> {
    pub fn get_addresses(self) -> (&'a str, &'a str) {
        match self {
            Self::Gmail => ("imap.gmail.com:993", "smtp.gmail.com:587"),
            Self::ICloud => ("imap.mail.me.com:993", "smtp.mail.me.com:587"),
            Self::Outlook => ("outlook.office365.com:993", "smtp-mail.outlook.com:587"),
            Self::Yahoo => ("imap.mail.yahoo.com:993", "smtp.mail.yahoo.com:587"),
            Self::Custom(imap, smtp) => (imap, smtp),
        }
    }
}
