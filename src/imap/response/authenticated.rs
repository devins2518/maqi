use super::{flags::Flags, response::ImapResponse, Response};
use crate::imap::{error::ImapResult, tag::Tag, tokens::Token};

pub struct ListReponse<'a> {
    tag: Tag,
    inner: Vec<ListInner<'a>>,
    response: ImapResponse,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ListInner<'a> {
    flags: Vec<Flags>,
    hierarchy_delim: &'a str,
    name: &'a str,
}

impl<'a> Response for ListInner<'a> {
    fn convert(tokens: &[Token]) -> Self {
        let mut iter = tokens.into_iter();
        assert!(*iter.next().unwrap() == Token::STAR);
        assert!(*iter.next().unwrap() == Token::List);
        assert!(*iter.next().unwrap() == Token::LPAREN);
        let flags = Flags::from_tokens(&mut iter);
        let hierarchy_delim = unimplemented!();
        let name = unimplemented!();

        Self {
            flags,
            hierarchy_delim,
            name,
        }
    }
    fn is_err(&self) -> ImapResult<()> {
        // ListInners should always have an untagged response
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{super::flags::Flags, super::Response, ListInner};
    use crate::imap::scanner::Scanner;

    #[test]
    fn test_list_inner() {
        let test = [
            "* LIST (\\Noselect) \"/\" ~/Mail/foo",
            "* LIST () \"/\" INBOX",
            "* LIST () \"/\" Drafts",
        ];
        let parsed = [
            ListInner {
                flags: vec![Flags::NoSelect],
                hierarchy_delim: "/",
                name: "~/Mail/foo",
            },
            ListInner {
                flags: vec![],
                hierarchy_delim: "/",
                name: "INBOX",
            },
            ListInner {
                flags: vec![],
                hierarchy_delim: "/",
                name: "Drafts",
            },
        ];

        for (test, parsed) in test.into_iter().zip(parsed.into_iter()) {
            let mut s = Scanner::new(test);
            s.scan_tokens();
            assert_eq!(ListInner::convert(&s.tokens), parsed)
        }
    }
}
