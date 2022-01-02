use super::tokens::Token;
use std::{
    fmt::{self, Display},
    str::{self, FromStr},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Tag {
    Real(TagRepr),
    Untagged,
    ClientContinuation, // Server requesting client sends more
}

impl From<&Token> for Tag {
    fn from(t: &Token) -> Self {
        match t {
            Token::Other(s) => Tag::Real(TagRepr::from(s)),
            Token::STAR => Tag::Untagged,
            Token::PLUS => Tag::ClientContinuation,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TagRepr {
    alpha: char,
    numeric: u16,
}

impl TagRepr {
    pub fn new() -> Self {
        Self {
            alpha: 'A',
            numeric: 0001,
        }
    }
    pub fn inc(&mut self) {
        if self.numeric == 9999 {
            self.alpha = if self.alpha == 'Z' {
                'A'
            } else {
                std::char::from_u32(self.alpha as u32 + 1).unwrap_or(self.alpha)
            };
            self.numeric = 1;
        } else {
            self.numeric += 1;
        }
    }
}

impl<T> From<T> for TagRepr
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let s = s.as_ref();
        let c = s.as_bytes();
        Self {
            alpha: char::from(c[0]),
            numeric: u16::from_str(str::from_utf8(&c[1..]).unwrap()).unwrap(),
        }
    }
}

impl Display for TagRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}{:>04}", self.alpha, self.numeric))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tag() {
        let mut t = TagRepr::new();
        // Increase number
        for i in 1..=9999 {
            assert_eq!(format!("A{:>04}", i), format!("{}", t));
            t.inc();
        }

        // Increase letter
        assert_eq!(String::from("B0001"), format!("{}", t));

        // Wraparound
        for _ in 0..(25 * 9999) {
            t.inc();
        }
        assert_eq!(String::from("A0001"), format!("{}", t));
    }
}
