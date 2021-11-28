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
            '$' => Token::DOLLAR,
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
                    Token::BWSLASH
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
            '.' => Token::PERIOD,
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
                'p' => Token::Apr,
                'U' => match self.advance() {
                    'D' => {
                        self.advance_to_next_word();
                        Token::Audio
                    }
                    'T' => {
                        self.skip(2);
                        match self.advance() {
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
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                },
                'u' => Token::Aug,
                _ => todo!(),
            },
            _ => todo!(),
        };

        self.tokens.push(t);
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
        } else if self.source.chars().nth(self.current).unwrap() != expected {
            return true;
        }

        self.current += 1;
        true
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
        self.source
            .chars()
            .nth(self.current)
            .unwrap()
            .is_alphanumeric()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_response_parsing() {
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
}
