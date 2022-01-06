use super::{
    flags::Flags,
    response::ImapResponse,
    scanner::{Scan, Scanner},
    Response,
};
use crate::imap::{error::ImapResult, tag::Tag};

pub struct ListReponse {
    tag: Tag,
    inner: Vec<ListInner>,
    response: ImapResponse,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ListInner {
    flags: Vec<Flags>,
    hierarchy_delim: String,
    name: String,
}

impl Scan for ListInner {
    fn scan(s: &str, scanner: Scanner) -> Self {
        let (rest, _) = scanner.scan_tag(s).unwrap();
        let (rest, _) = scanner.scan_word(rest, "LIST").unwrap();
        println!("'{}'", rest);
        let (rest, flags) = scanner.scan_flags(rest).unwrap();
        let (rest, _) = scanner.scan_space(rest).unwrap();
        let (rest, hierarchy_delim) = scanner.scan_quotes(rest).unwrap();
        let (name, _) = scanner.scan_space(rest).unwrap();

        Self {
            flags,
            hierarchy_delim: hierarchy_delim.to_string(),
            name: name.to_string(),
        }
    }
}

impl Response for ListInner {
    fn is_err(&self) -> ImapResult<()> {
        // ListInners should always have an untagged response
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{super::flags::Flags, super::Response, ListInner};

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
            assert_eq!(ListInner::convert(test), parsed)
        }
    }
}
