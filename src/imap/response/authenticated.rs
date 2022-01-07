use super::{
    flags::Flags,
    response::ImapResponse,
    scanner::{Scan, Scanner},
    Response,
};
use crate::imap::{
    error::{ImapError, ImapResult},
    tag::Tag,
};

pub struct ListResponse {
    tag: Option<Tag>,
    pub inner: Vec<ListInner>,
    response: Option<ImapResponse>,
}

impl Response for ListResponse {
    fn is_err(&self) -> ImapResult<()> {
        // ListInners should always have an untagged response
        unimplemented!()
    }

    fn should_continue(v: &[u8]) -> bool {
        // if let Some('*') =
        unimplemented!()
    }
}

impl Scan for ListResponse {
    fn scan(s: &str, scanner: Scanner) -> Result<Self, ImapError> {
        let inner: Vec<ListInner> = s
            .lines()
            // TODO!!!!: peekable_map_while
            .map_while(|x| ListInner::scan(x, Scanner).ok())
            .collect();
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ListInner {
    flags: Vec<Flags>,
    pub hierarchy_delim: String,
    pub name: String,
}

impl Scan for ListInner {
    fn scan(s: &str, scanner: Scanner) -> Result<Self, ImapError> {
        let (rest, _) = scanner.scan_untagged(s)?;
        let (rest, _) = scanner.scan_word(rest, "LIST").unwrap();
        println!("'{}'", rest);
        let (rest, flags) = scanner.scan_flags(rest).unwrap();
        let (rest, _) = scanner.scan_space(rest).unwrap();
        let (rest, hierarchy_delim) = scanner.scan_quotes(rest).unwrap();
        let (name, _) = scanner.scan_space(rest).unwrap();

        Ok(Self {
            flags,
            hierarchy_delim: hierarchy_delim.to_string(),
            name: name.to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::imap::response::{Scan, Scanner};

    use super::{super::flags::Flags, ListInner};

    #[test]
    fn test_list_inner() {
        let test = [
            "* LIST (\\Noselect \\NonExistent) \"/\" ~/Mail/foo",
            "* LIST () \"/\" INBOX",
            "* LIST () \"/\" Drafts",
        ];
        let parsed = [
            ListInner {
                flags: vec![Flags::NoSelect, Flags::NonExistent],
                hierarchy_delim: String::from("/"),
                name: String::from("~/Mail/foo"),
            },
            ListInner {
                flags: vec![],
                hierarchy_delim: String::from("/"),
                name: String::from("INBOX"),
            },
            ListInner {
                flags: vec![],
                hierarchy_delim: String::from("/"),
                name: String::from("Drafts"),
            },
        ];

        for (test, parsed) in test.into_iter().zip(parsed.into_iter()) {
            assert_eq!(ListInner::scan(test, Scanner).unwrap(), parsed)
        }
    }
}
