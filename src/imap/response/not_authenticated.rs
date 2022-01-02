use super::{result::ImapResponse, Response};
use crate::imap::{
    error::{ImapError, ImapResult},
    tag::Tag,
    tokens::Token,
};

#[derive(Debug, PartialEq, Eq)]
pub struct LoginResponse {
    tag: Tag,
    response: ImapResponse,
}

// TODO: record AUTHENTICATIONFAILED as a separate error
impl Response for LoginResponse {
    fn receive<'a>(tokens: &[Token]) -> Self {
        let tag = Tag::from(&tokens[0]);
        let response = ImapResponse::from(&tokens[1]);
        Self { tag, response }
    }
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
    use crate::imap::{scanner::Scanner, tag::TagRepr};

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
            let mut s = Scanner::new(test);
            s.scan_tokens();
            assert_eq!(LoginResponse::receive(&s.tokens), parsed)
        }
    }
}
