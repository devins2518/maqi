use super::{super::tag::Tag, flags::Flags, response::ImapResponse};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alphanumeric1, char},
    combinator::opt,
    multi::many0,
    sequence::{delimited, preceded, terminated},
    IResult,
};

pub struct Scanner;

impl<'a> Scanner {
    pub fn scan_tag(&self, s: &'a str) -> IResult<&'a str, Tag> {
        terminated(alt((tag("*"), alt((tag("+"), alphanumeric1)))), tag(" "))(s)
            .map(|x| (x.0, Tag::from(x.1)))
    }

    pub fn scan_space(&self, s: &'a str) -> IResult<&'a str, &'a str> {
        tag(" ")(s)
    }

    pub fn scan_flags(&self, s: &'a str) -> IResult<&'a str, Vec<Flags>> {
        let (rest, inner) = self.scan_parens(s)?;
        println!("in {}", inner);
        let flags = many0(preceded(
            tag("\\"),
            terminated(alphanumeric1, opt(tag(" "))),
        ))(inner)?
        .1
        .iter()
        .map(|x| Flags::from(*x))
        .collect();
        Ok((rest, flags))
    }

    pub fn scan_response(&self, s: &'a str) -> IResult<&'a str, ImapResponse> {
        terminated(alphanumeric1, tag(" "))(s).map(|x| (x.0, ImapResponse::from(x.1)))
    }

    pub fn scan_word(&self, s: &'a str, word: &'static str) -> IResult<&'a str, &'a str> {
        terminated(tag(word), tag(" "))(s)
    }

    pub fn scan_parens(&self, s: &'a str) -> IResult<&'a str, &'a str> {
        delimited(char('('), opt(is_not(")")), char(')'))(s).map(|x| (x.0, x.1.unwrap_or("")))
    }

    pub fn scan_braces(&self, s: &'a str) -> IResult<&'a str, &'a str> {
        delimited(char('{'), opt(is_not("}")), char('}'))(s).map(|x| (x.0, x.1.unwrap_or("")))
    }

    pub fn scan_brackets(&self, s: &'a str) -> IResult<&'a str, &'a str> {
        delimited(char('['), opt(is_not("]")), char(']'))(s).map(|x| (x.0, x.1.unwrap_or("")))
    }

    pub fn scan_quotes(&self, s: &'a str) -> IResult<&'a str, &'a str> {
        delimited(char('"'), opt(is_not("\"")), char('"'))(s).map(|x| (x.0, x.1.unwrap_or("")))
    }
}

pub trait Scan {
    fn scan(s: &str, scanner: Scanner) -> Self;
}

#[cfg(test)]
mod test {
    use crate::imap::tag::TagRepr;

    use super::*;

    #[test]
    fn test_tag_scanning() {
        let scanner = Scanner;
        assert_eq!(
            scanner.scan_tag("* OK [CAPABILITY IMAP4rev2 STARTTLS AUTH=GSSAPI]"),
            Ok((
                "OK [CAPABILITY IMAP4rev2 STARTTLS AUTH=GSSAPI]",
                Tag::Untagged
            ))
        );
        assert_eq!(
            scanner.scan_tag("A01 OK STARTTLS complete"),
            Ok(("OK STARTTLS complete", Tag::Real(TagRepr::from("A01"))))
        );
    }

    #[test]
    fn test_paren() {
        let scanner = Scanner;
        assert_eq!(scanner.scan_parens("()hi"), Ok(("hi", "")));
        assert_eq!(scanner.scan_parens("(hi)out"), Ok(("out", "hi")));
        assert_eq!(scanner.scan_parens("(hi) out"), Ok((" out", "hi")));
    }

    /*
    #[test]
    fn test_full_responses() {
        let rs = [
            "* OK [CAPABILITY IMAP4rev2 STARTTLS AUTH=GSSAPI]",
            "A01 OK STARTTLS complete",
            "* CAPABILITY IMAP4rev2 AUTH=GSSAPI AUTH=PLA",
            "* FLAGS (\\Answered \\Flagged \\Deleted \\Seen \\Draft)",
            "* LIST () \"/\" blurdybloop",
            "A932 OK [READ-ONLY] EXAMINE complete",
            "* CAPABILITY IMAP4rev1 UNSELECT IDLE NAMESPACE QUOTA ID XLIST CHILDREN X-GM-EXT-1 XYZZY SASL-IR AUTH=XOAUTH2 AUTH=PLAIN AUTH=PLAIN-CLIENTTOKEN AUTH=OAUTHBEARER AUTH=XOAUTH"
        ];
        let ts = [
            vec![
                Token::STAR,
                Token::SP,
                Token::Ok,
                Token::SP,
                Token::LBRACKET,
                Token::Capability,
                Token::SP,
                Token::IMAP4Rev2,
                Token::SP,
                Token::StartTLS,
                Token::SP,
                Token::Auth,
                Token::EQUAL,
                Token::Other(String::from("GSSAPI")),
                Token::RBRACKET,
            ],
            vec![
                Token::Other(String::from("A01")),
                Token::SP,
                Token::Ok,
                Token::SP,
                Token::StartTLS,
                Token::SP,
                Token::Other(String::from("complete")),
            ],
            vec![
                Token::STAR,
                Token::SP,
                Token::Capability,
                Token::SP,
                Token::IMAP4Rev2,
                Token::SP,
                Token::Auth,
                Token::EQUAL,
                Token::Other(String::from("GSSAPI")),
                Token::SP,
                Token::Auth,
                Token::EQUAL,
                Token::Other(String::from("PLA")),
            ],
            vec![
                Token::STAR,
                Token::SP,
                Token::Flags,
                Token::SP,
                Token::LPAREN,
                Token::BWSLASH,
                Token::BWSlashAnswered,
                Token::SP,
                Token::BWSLASH,
                Token::BWSlashFlagged,
                Token::SP,
                Token::BWSLASH,
                Token::BWSlashDeleted,
                Token::SP,
                Token::BWSLASH,
                Token::BWSlashSeen,
                Token::SP,
                Token::BWSLASH,
                Token::BWSlashDraft,
                Token::RPAREN,
            ],
            vec![
                Token::STAR,
                Token::SP,
                Token::List,
                Token::SP,
                Token::LPAREN,
                Token::RPAREN,
                Token::SP,
                Token::DQUOTE,
                Token::FWSLASH,
                Token::DQUOTE,
                Token::SP,
                Token::Other(String::from("blurdybloop")),
            ],
            vec![
                Token::Other(String::from("A932")),
                Token::SP,
                Token::Ok,
                Token::SP,
                Token::LBRACKET,
                Token::Read,
                Token::HYPHEN,
                Token::Only,
                Token::RBRACKET,
                Token::SP,
                Token::Examine,
                Token::SP,
                Token::Other(String::from("complete")),
            ],
            vec![
                Token::STAR,
                Token::SP,
                Token::Capability,
                Token::SP,
                Token::IMAP4Rev1,
                Token::SP,
                Token::Unselect,
                Token::SP,
                Token::Idle,
                Token::SP,
                Token::Namespace,
                Token::SP,
                Token::Other(String::from("QUOTA")),
                Token::SP,
                Token::Other(String::from("ID")),
                Token::SP,
                Token::Other(String::from("XLIST")),
                Token::SP,
                Token::Children,
                Token::SP,
                Token::Other(String::from("X")),
                Token::HYPHEN,
                Token::Other(String::from("GM")),
                Token::HYPHEN,
                Token::Other(String::from("EXT")),
                Token::HYPHEN,
                Token::Other(String::from("1")),
                Token::SP,
                Token::Other(String::from("XYZZY")),
                Token::SP,
                Token::Other(String::from("SASL")),
                Token::HYPHEN,
                Token::Other(String::from("IR")),
                Token::SP,
                Token::Auth,
                Token::EQUAL,
                Token::Other(String::from("XOAUTH2")),
                Token::SP,
                Token::Auth,
                Token::EQUAL,
                Token::Other(String::from("PLAIN")),
                Token::SP,
                Token::Auth,
                Token::EQUAL,
                Token::Other(String::from("PLAIN")),
                Token::SP,
                Token::HYPHEN,
                Token::Other(String::from("CLIENTTOKEN")),
                Token::SP,
                Token::Auth,
                Token::EQUAL,
                Token::Other(String::from("OAUTHBEARER")),
                Token::SP,
                Token::Auth,
                Token::EQUAL,
                Token::Other(String::from("XOAUTH")),
            ],
        ];

        for (i, s) in rs.iter().enumerate() {
            let mut scanner = Scanner::new(s);
            scanner.scan_tokens();
            assert_eq!(scanner.tokens, ts[i])
        }
    }

    #[test]
    fn test_autheq() {
        let rs = "AUTH=GSSAPI";

        let mut scanner = Scanner::new(rs);
        scanner.scan_tokens();

        assert_eq!(
            scanner.tokens,
            vec![
                Token::Auth,
                Token::EQUAL,
                Token::Other(String::from("GSSAPI"))
            ]
        );
    }*/
}
