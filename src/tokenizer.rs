use crate::token::Token;

pub trait Tokenize {
    fn tokenize(&mut self) -> &Vec<Token>;
}

pub struct Tokenizer {
    source: String,
    current: usize,
    start: usize,
    tokens: Vec<Token>,
    line: usize,
}

impl Tokenize for Tokenizer {
    fn tokenize(&mut self) -> &Vec<Token> {
        self.current = 0;
        self.start = 0;
        self.line = 1;
        self.tokens.clear();

        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::Eof);

        &self.tokens
    }
}

impl Tokenizer {
    pub fn new(source: String) -> Self {
        Tokenizer {
            source,
            current: 0,
            line: 0,
            start: 0,
            tokens: Vec::new(),
        }
    }

    fn scan_token(&mut self) {
        let chr = self.advance();
        match chr {
            Some(' ' | '\r' | '\t') => (),
            Some('\n') => self.line += 1,
            Some('<') => {
                if let Some('=') = self.advance() {
                    self.add_token(Token::LessOrEqual)
                } else {
                    self.add_token(Token::Less)
                }
            }
            Some('>') => {
                if let Some('=') = self.advance() {
                    self.add_token(Token::GreaterOrEqual)
                } else {
                    self.add_token(Token::Greater)
                }
            }
            Some('(') => self.add_token(Token::LeftParen),
            Some(')') => self.add_token(Token::RightParen),
            Some('+') => self.add_token(Token::Plus),
            Some('-') => self.add_token(Token::Minus),
            Some('*') => self.add_token(Token::Asterisk),
            Some('%') => self.add_token(Token::Percent),
            Some('"') => self.string(),
            Some('!') => {
                if let Some('=') = self.advance() {
                    self.add_token(Token::NotEqual);
                } else {
                    self.add_token(Token::Exclamation);
                }
            }
            Some('|') => {
                if let Some('|') = self.advance() {
                    self.add_token(Token::Or);
                } else {
                    panic!("unexpected character {}", self.previous())
                }
            }
            Some('&') => {
                if let Some('&') = self.advance() {
                    self.add_token(Token::And);
                } else {
                    panic!("unexpected character {}", self.previous())
                }
            }
            Some('=') => match self.peek() {
                Some('=') => {
                    self.add_token(Token::DoubleEqual);
                    self.advance();
                }
                Some('>') => {
                    self.add_token(Token::Arrow);
                    self.advance();
                }
                _ => self.add_token(Token::Equal),
            },
            Some('{') => self.add_token(Token::LCurly),
            Some('}') => self.add_token(Token::RCurly),
            Some(',') => self.add_token(Token::Comma),
            Some(':') => self.add_token(Token::Colon),
            Some('/') => {
                if let Some('/') = self.peek() {
                    while !matches!(self.peek(), Some('\n')) && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Token::Slash);
                }
            }
            Some(c) if c.is_alphabetic() => self.identifier(),
            Some(c) if c.is_numeric() => self.numeric(),
            Some(c) => panic!("Unexpected character '{}'", c),
            _ => panic!("End of stream"),
        };
    }

    fn string(&mut self) {
        let mut literal = String::new();

        while !matches!(self.peek(), Some('"')) {
            if let Some(chr) = self.advance() {
                literal.push(chr);
            } else {
                break;
            }
        }
        self.advance();
        self.add_token(Token::String(literal));
    }

    fn numeric(&mut self) {
        let mut literal = String::new();
        literal.push(self.previous());

        while let Some(chr) = self.peek() {
            if chr.is_numeric() || chr == '.' {
                literal.push(chr);
                self.advance();
            } else {
                break;
            }
        }

        self.add_token(Token::Numeric(
            literal.parse().expect("Error parsing number"),
        ));
    }

    fn identifier(&mut self) {
        let mut literal = String::new();
        literal.push(self.previous());

        while let Some(chr) = self.peek() {
            if chr.is_alphanumeric() {
                literal.push(chr);
                self.advance();
            } else {
                break;
            }
        }

        match literal.as_str() {
            "if" => self.add_token(Token::If),
            "while" => self.add_token(Token::While),
            "true" => self.add_token(Token::True),
            "false" => self.add_token(Token::False),
            "break" => self.add_token(Token::Break),
            "else" => self.add_token(Token::Else),
            _ => self.add_token(Token::Identifier(literal)),
        };
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn advance(&mut self) -> Option<char> {
        let chr = self.peek();
        self.current += 1;
        chr
    }

    fn previous(&mut self) -> char {
        let chr = self.source.chars().nth(self.current - 1).unwrap();
        chr
    }

    fn peek(&mut self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }
}
