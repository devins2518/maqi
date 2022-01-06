use super::{
    response::ImapResponse,
    scanner::{Scan, Scanner},
    Response,
};
use crate::imap::{
    error::{ImapError, ImapResult},
    tag::Tag,
};

#[derive(Debug, PartialEq, Eq)]
pub struct LoginResponse {
    tag: Tag,
    response: ImapResponse,
}

impl Scan for LoginResponse {
    fn scan(s: &str, scanner: Scanner) -> Self {
        let (rest, tag) = scanner.scan_tag(s).unwrap();
        let (rest, response) = scanner.scan_response(rest).unwrap();

        Self { tag, response }
    }
}

// TODO: record AUTHENTICATIONFAILED as a separate error
impl Response for LoginResponse {
    fn is_err(&self) -> ImapResult<()> {
        match self.response {
            ImapResponse::No => Err(ImapError::Bad),
            ImapResponse::Bad => Err(ImapError::Bad),
            ImapResponse::Preauth => Err(ImapError::Preauth),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::imap::tag::TagRepr;

    #[test]
    fn test_login() {
        let test = [
            "A0001 BAD tag LOGIN username password",
            "A0001 NO [AUTHENTICATIONFAILED] Authentication Failed",
            "A0001 OK user drsingh2518 logged in",
        ];
        let parsed = [
            LoginResponse {
                tag: Tag::Real(TagRepr::from("A0001")),
                response: ImapResponse::Bad,
            },
            LoginResponse {
                tag: Tag::Real(TagRepr::from("A0001")),
                response: ImapResponse::No,
            },
            LoginResponse {
                tag: Tag::Real(TagRepr::from("A0001")),
                response: ImapResponse::Ok,
            },
        ];

        for (test, parsed) in test.into_iter().zip(parsed.into_iter()) {
            assert_eq!(LoginResponse::convert(test), parsed)
        }
    }
}
