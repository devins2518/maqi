use super::parser::Token;

struct Scanner<'str> {
    // TODO: Change to scan on a stream
    source: &'str str,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
}

impl<'str> Scanner<'str> {
    pub fn new(source: &'str str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        let t = match c {
            '$' => {
                if !self.is_alphanumeric() {
                    Token::DOLLAR
                } else {
                    let c = self.advance();
                    self.advance_to_next_word();
                    match c {
                        'F' => Token::DollarForwarded,
                        'J' => Token::DollarJunk,
                        'M' => Token::DollarMDNSent,
                        'N' => Token::DollarNotJunk,
                        'P' => Token::DollarPhishing,
                        _ => unreachable!(),
                    }
                }
            }
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '[' => Token::LBRACKET,
            ']' => Token::RBRACKET,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '<' => Token::LANGLE,
            '>' => Token::RANGLE,
            '+' => Token::PLUS,
            '/' => Token::FWSLASH,
            '\\' => {
                if self.next('*') {
                    Token::BWSLASHSTAR
                } else {
                    if !self.is_alphanumeric() {
                        Token::BWSLASH
                    } else {
                        match self.advance() {
                            'A' => {
                                self.advance_to_next_word();
                                Token::BWSlashAnswered
                            }
                            'D' => {
                                if self.next('e') {
                                    self.advance_to_next_word();
                                    Token::BWSlashDeleted
                                } else {
                                    self.advance_to_next_word();
                                    Token::BWSlashDraft
                                }
                            }
                            'F' => {
                                self.advance_to_next_word();
                                Token::BWSlashFlagged
                            }
                            'H' => {
                                self.skip(2);
                                if self.next('C') {
                                    self.advance_to_next_word();
                                    Token::BWSlashHasChildren
                                } else {
                                    self.advance_to_next_word();
                                    Token::BWSlashHasNoChildren
                                }
                            }
                            'M' => {
                                self.advance_to_next_word();
                                Token::BWSlashMarked
                            }
                            'N' => {
                                self.skip(1);
                                let c = self.advance();
                                self.advance_to_next_word();
                                match c {
                                    'i' => Token::BWSlashNoInferiors,
                                    's' => Token::BWSlashNoSelect,
                                    'n' => Token::BWSlashNonExistent,
                                    _ => unreachable!(),
                                }
                            }
                            'R' => {
                                self.skip(1);
                                let c = self.advance();
                                self.advance_to_next_word();
                                match c {
                                    'c' => Token::BWSlashRecent,
                                    'm' => Token::BWSlashRemote,
                                    _ => unreachable!(),
                                }
                            }
                            'S' => {
                                let c = self.advance();
                                self.advance_to_next_word();
                                match c {
                                    'e' => Token::BWSlashSeen,
                                    'u' => Token::BWSlashSubscribed,
                                    _ => unreachable!(),
                                }
                            }
                            'U' => {
                                self.advance_to_next_word();
                                Token::BWSlashUnmarked
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
            '=' => {
                if self.next('=') {
                    Token::DBLEQUAL
                } else {
                    Token::EQUAL
                }
            }
            '-' => Token::HYPHEN,
            '*' => Token::STAR,
            '%' => Token::PERCENT,
            // TODO: possibly wrong
            '~' => Token::APPROXLBRACE,
            '.' => {
                if !self.is_alphanumeric() {
                    Token::PERIOD
                } else {
                    let c = self.advance();
                    self.advance_to_next_word();
                    match c {
                        'N' => Token::DotNot,
                        'P' => Token::DotPeek,
                        'S' => Token::DotSilent,
                        _ => unreachable!(),
                    }
                }
            }
            ':' => Token::COLON,
            ',' => Token::COMMA,
            '_' => Token::UNDERSCORE,
            // TODO: possibly wrong
            '\r' => Token::CRLF,
            ' ' => Token::SP,
            'A' => match self.advance() {
                'L' => {
                    let c = self.peek();
                    self.advance_to_next_word();
                    match c {
                        'E' => Token::Alert,
                        'L' => Token::All,
                        'R' => Token::AlreadyExists,
                        _ => unreachable!(),
                    }
                }
                'N' => {
                    self.advance_to_next_word();
                    Token::Answered
                }
                'P' => {
                    self.skip(2);
                    match self.advance() {
                        'E' => {
                            self.skip(2);
                            let c = self.advance();
                            self.advance_to_next_word();
                            match c {
                                'U' => Token::AppendUID,
                                _ => Token::Append,
                            }
                        }
                        'L' => {
                            self.advance_to_next_word();
                            Token::Application
                        }
                        _ => unreachable!(),
                    }
                }
                'p' => {
                    self.advance_to_next_word();
                    Token::Apr
                }
                'U' => match self.advance() {
                    'D' => {
                        self.advance_to_next_word();
                        Token::Audio
                    }
                    'T' => {
                        self.skip(1);
                        match self.advance() {
                            '=' => {
                                let current = self.current;
                                self.advance_to_next_word();
                                let diff = self.current - current;
                                Token::AuthEq(
                                    self.source
                                        .chars()
                                        .skip(current)
                                        .take(diff)
                                        .collect::<String>(),
                                )
                            }
                            'E' => {
                                self.skip(7);
                                let c = self.advance();
                                self.advance_to_next_word();
                                match c {
                                    'E' => Token::Authenticate,
                                    'I' => Token::AuthenticationFailed,
                                    _ => unreachable!(),
                                }
                            }
                            'O' => {
                                self.advance_to_next_word();
                                Token::AuthorizationFailed
                            }
                            x => unreachable!("found {}", x),
                        }
                    }
                    _ => unreachable!(),
                },
                'u' => {
                    self.advance_to_next_word();
                    Token::Aug
                }
                x => todo!("todo: {}", x),
            },
            'B' => match self.advance() {
                'A' => match self.advance() {
                    'D' => {
                        if self.is_alphanumeric() {
                            self.advance_to_next_word();
                            Token::BadCharset
                        } else {
                            self.advance_to_next_word();
                            Token::Bad
                        }
                    }
                    'S' => {
                        self.advance_to_next_word();
                        Token::Base64
                    }
                    _ => unimplemented!(),
                },
                'C' => {
                    self.advance_to_next_word();
                    Token::Bcc
                }
                'E' => {
                    self.advance_to_next_word();
                    Token::Before
                }
                'I' => {
                    self.skip(4);
                    if self.next('.') {
                        self.advance_to_next_word();
                        Token::BinaryDotSize
                    } else {
                        self.advance_to_next_word();
                        Token::Binary
                    }
                }
                'O' => {
                    self.skip(2);
                    if self.next('.') {
                        self.advance_to_next_word();
                        Token::BodyDotPeek
                    } else {
                        self.advance_to_next_word();
                        Token::Body
                    }
                }
                'Y' => {
                    self.advance_to_next_word();
                    Token::Bye
                }
                _ => unreachable!(),
            },
            'C' => match self.advance() {
                'A' => {
                    let c = self.advance();
                    self.advance_to_next_word();
                    match c {
                        'N' => Token::Cannot,
                        'P' => Token::Capability,
                        _ => unreachable!(),
                    }
                }
                'C' => {
                    self.advance_to_next_word();
                    Token::Cc
                }
                'H' => match self.advance() {
                    'A' => {
                        self.advance_to_next_word();
                        Token::Charset
                    }
                    'I' => {
                        self.skip(2);
                        let c = self.advance();
                        self.advance_to_next_word();
                        match c {
                            'I' => Token::ChildInfo,
                            'R' => Token::Children,
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                },
                'L' => match self.advance() {
                    'I' => {
                        self.advance_to_next_word();
                        Token::ClientBug
                    }
                    'O' => {
                        self.skip(2);
                        if self.next('D') {
                            self.advance_to_next_word();
                            Token::Closed
                        } else {
                            self.advance_to_next_word();
                            Token::Close
                        }
                    }
                    _ => unreachable!(),
                },
                'O' => match self.advance() {
                    'N' => {
                        self.advance_to_next_word();
                        Token::ContactAdmin
                    }
                    'P' => {
                        self.skip(2);
                        if self.next('U') {
                            self.advance_to_next_word();
                            Token::CopyUID
                        } else {
                            self.advance_to_next_word();
                            Token::Copy
                        }
                    }
                    'R' => {
                        self.advance_to_next_word();
                        Token::Corruption
                    }
                    'U' => {
                        self.advance_to_next_word();
                        Token::Count
                    }
                    _ => unreachable!(),
                },
                'R' => {
                    self.advance_to_next_word();
                    Token::Create
                }
                _ => unreachable!(),
            },
            'D' => match self.advance() {
                'e' => {
                    self.advance_to_next_word();
                    Token::Dec
                }
                'E' => {
                    self.skip(4);
                    if self.next('D') {
                        self.advance_to_next_word();
                        Token::Deleted
                    } else {
                        self.advance_to_next_word();
                        Token::Delete
                    }
                }
                'O' => {
                    self.advance_to_next_word();
                    Token::Done
                }
                'R' => {
                    self.advance_to_next_word();
                    Token::Draft
                }
                _ => unreachable!(),
            },
            'O' => {
                let c = self.advance();
                self.advance_to_next_word();
                match c {
                    'c' => Token::Oct,
                    'K' => Token::Ok,
                    'L' => Token::OldName,
                    'N' => Token::On,
                    'R' => Token::Or,
                    'V' => Token::OverQuota,
                    _ => unreachable!(),
                }
            }
            'E' => match self.advance() {
                'S' => {
                    self.advance_to_next_word();
                    Token::ESearch
                }
                'N' => match self.advance() {
                    'A' => {
                        self.skip(3);
                        if self.next('D') {
                            Token::Enabled
                        } else {
                            Token::Enable
                        }
                    }
                    'V' => {
                        self.advance_to_next_word();
                        Token::Envelope
                    }
                    _ => unreachable!(),
                },
                'X' => match self.advance() {
                    'A' => {
                        self.advance_to_next_word();
                        Token::Examine
                    }
                    'I' => {
                        self.advance_to_next_word();
                        Token::Exists
                    }
                    'P' => match self.advance() {
                        'I' => {
                            self.advance_to_next_word();
                            Token::Expired
                        }
                        'U' => {
                            self.skip(3);
                            if self.next('I') {
                                self.advance_to_next_word();
                                Token::ExpungeIssued
                            } else {
                                self.advance_to_next_word();
                                Token::Expunge
                            }
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            'F' => match self.advance() {
                'A' => {
                    self.advance_to_next_word();
                    Token::Fast
                }
                'e' => {
                    self.advance_to_next_word();
                    Token::Feb
                }
                'E' => {
                    self.advance_to_next_word();
                    Token::Fetch
                }
                'L' => {
                    self.skip(2);
                    let c = self.advance();
                    self.advance_to_next_word();
                    match c {
                        'G' => Token::Flagged,
                        'S' => Token::Flags,
                        _ => unreachable!(),
                    }
                }
                'O' => {
                    self.advance_to_next_word();
                    Token::Font
                }
                'R' => {
                    self.advance_to_next_word();
                    Token::From
                }
                'U' => {
                    self.advance_to_next_word();
                    Token::Full
                }
                _ => unreachable!(),
            },
            'G' => {
                self.advance_to_next_word();
                Token::Global
            }
            'H' => match self.advance() {
                'A' => {
                    self.advance_to_next_word();
                    Token::HasChildren
                }
                'E' => {
                    self.advance_to_next_word();
                    // TODO: maybe wrong
                    if self.next('.') {
                        Token::HeaderDotFields
                    } else {
                        Token::Header
                    }
                }
                _ => unreachable!(),
            },
            'I' => match self.advance() {
                'M' => {
                    self.skip(1);
                    let c = self.advance();
                    self.advance_to_next_word();
                    match c {
                        'P' => Token::IMAP4Rev2,
                        'G' => Token::Image,
                        _ => unreachable!(),
                    }
                }
                'D' => {
                    self.advance_to_next_word();
                    Token::Idle
                }
                'N' => {
                    let c = self.advance();
                    self.advance_to_next_word();
                    match c {
                        'U' => Token::InUse,
                        'B' => Token::Inbox,
                        'T' => Token::Internaldate,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            },
            'J' => {
                let c = self.advance();
                let n = self.advance();
                match (c, n) {
                    ('a', 'n') => Token::Jan,
                    ('u', 'l') => Token::Jul,
                    ('u', 'n') => Token::Jun,
                    _ => unreachable!(),
                }
            }
            'K' => {
                self.advance_to_next_word();
                Token::Keyword
            }
            'L' => match self.advance() {
                'A' => {
                    self.advance_to_next_word();
                    Token::Larger
                }
                'I' => {
                    if self.next('M') {
                        self.advance_to_next_word();
                        Token::Limit
                    } else {
                        self.advance_to_next_word();
                        Token::List
                    }
                }
                'O' => {
                    self.skip(1);
                    if self.next('I') {
                        self.advance_to_next_word();
                        Token::Login
                    } else {
                        self.advance_to_next_word();
                        Token::Logout
                    }
                }
                _ => unreachable!(),
            },
            'M' => match self.advance() {
                'a' => {
                    if self.next('r') {
                        self.advance_to_next_word();
                        Token::Mar
                    } else {
                        self.advance_to_next_word();
                        Token::May
                    }
                }
                'A' => {
                    self.advance_to_next_word();
                    Token::Max
                }
                'E' => {
                    self.skip(5);
                    if self.next('S') {
                        self.advance_to_next_word();
                        Token::Messages
                    } else {
                        self.advance_to_next_word();
                        Token::Message
                    }
                }
                'I' => {
                    if self.next('M') {
                        self.advance_to_next_word();
                        Token::Mime
                    } else {
                        self.advance_to_next_word();
                        Token::Min
                    }
                }
                _ => unreachable!(),
            },
            'N' => match self.advance() {
                'A' => {
                    self.advance_to_next_word();
                    Token::Namespace
                }
                'I' => {
                    self.advance_to_next_word();
                    Token::Nil
                }
                'O' => {
                    if self.next('P') {
                        self.advance_to_next_word();
                        Token::NoPerm
                    } else if self.next('N') {
                        self.advance_to_next_word();
                        Token::NonExistent
                    } else if self.next('O') {
                        self.advance_to_next_word();
                        Token::Noop
                    } else if self.next('T') {
                        self.advance_to_next_word();
                        Token::NotSaved
                    } else {
                        self.advance_to_next_word();
                        Token::No
                    }
                }
                'o' => {
                    self.advance_to_next_word();
                    Token::Nov
                }
            },
            '8' => {
                self.advance_to_next_word();
                Token::EightBit
            }
            x => todo!("didn't handle {}", x),
        };

        self.tokens.push(t);
        println!("{:#?}", self.tokens);
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        return c;
    }

    fn skip(&mut self, n: usize) {
        self.current += n;
    }

    fn next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        } else {
            if let Some(x) = self.source.chars().nth(self.current) {
                if x != expected {
                    false
                } else {
                    self.current += 1;
                    true
                }
            } else {
                false
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn advance_to_next_word(&mut self) {
        while self.is_alphanumeric() {
            self.advance();
        }
    }
    fn is_alphanumeric(&self) -> bool {
        if let Some(x) = self.source.chars().nth(self.current) {
            x.is_alphanumeric()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_responses() {
        let rs = [
            "* OK [CAPABILITY IMAP4rev2 STARTTLS AUTH=GSSAPI]",
            "A01 OK STARTTLS complete",
            "* CAPABILITY IMAP4rev2 AUTH=GSSAPI AUTH=PLA",
            "* FLAGS (\\Answered \\Flagged \\Deleted \\Seen \\Draft",
            "* LIST () \" / \" blurdybloop",
            "A932 OK [READ-ONLY] EXAMINE complet",
        ];

        let mut scanner = Scanner::new(rs[0]);
        scanner.scan_tokens();

        assert_eq!(
            scanner.tokens,
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
                Token::AuthEq(String::from("GSSAPI")),
                Token::RBRACKET
            ]
        );
    }

    #[test]
    fn test_autheq() {
        let rs = "AUTH=GSSAPI";

        let mut scanner = Scanner::new(rs);
        scanner.scan_tokens();

        assert_eq!(scanner.tokens, vec![Token::AuthEq(String::from("GSSAPI"))]);
    }
}
