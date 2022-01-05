use std::slice::Iter;

use crate::imap::tokens::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum Flags {
    NonExistent,
    NoInferiors,
    NoSelect,
    HasChildren,
    HasNoChildren,
    Marked,
    Unmarked,
    Subscribed,
    Remote,
}

impl Flags {
    pub fn from_tokens(tokens: &mut Iter<Token>) -> Vec<Self> {
        tokens
            .filter(|token| **token != Token::BWSLASH)
            .map_while(|token| {
                if *token == Token::RPAREN {
                    None
                } else {
                    match *token {
                        Token::BWSlashNonExistent => Some(Self::NonExistent),
                        Token::BWSlashNoInferiors => Some(Self::NoInferiors),
                        Token::BWSlashNoSelect => Some(Self::NoSelect),
                        Token::BWSlashHasChildren => Some(Self::HasChildren),
                        Token::BWSlashHasNoChildren => Some(Self::HasNoChildren),
                        Token::BWSlashMarked => Some(Self::Marked),
                        Token::BWSlashUnmarked => Some(Self::Unmarked),
                        Token::BWSlashSubscribed => Some(Self::Subscribed),
                        Token::BWSlashRemote => Some(Self::Remote),
                        _ => unreachable!(),
                    }
                }
            })
            .collect()
    }
}
