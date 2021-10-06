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
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '<' => {
                if self.advance() == '=' {
                    self.add_token(Token::LessOrEqual)
                } else {
                    self.add_token(Token::Less)
                }
            }
            '>' => {
                if self.advance() == '=' {
                    self.add_token(Token::GreaterOrEqual)
                } else {
                    self.add_token(Token::Greater)
                }
            }
            '(' => self.add_token(Token::LeftParen),
            ')' => self.add_token(Token::RightParen),
            '+' => self.add_token(Token::Plus),
            '-' => self.add_token(Token::Minus),
            '*' => self.add_token(Token::Asterisk),
            '%' => self.add_token(Token::Percent),
            '"' => self.string(),
            '!' => {
                if self.advance() == '=' {
                    self.add_token(Token::NotEqual);
                } else {
                    self.add_token(Token::Exclamation);
                }
            }
            '|' => {
                if self.advance() == '|' {
                    self.add_token(Token::Or);
                } else {
                    panic!("unexpected character {}", self.previous())
                }
            }
            '&' => {
                if self.advance() == '&' {
                    self.add_token(Token::And);
                } else {
                    panic!("unexpected character {}", self.previous())
                }
            }
            '=' => match self.peek() {
                '=' => {
                    self.add_token(Token::DoubleEqual);
                    self.advance();
                }
                '>' => {
                    self.add_token(Token::Arrow);
                    self.advance();
                }
                _ => self.add_token(Token::Equal),
            },
            '{' => self.add_token(Token::LCurly),
            '}' => self.add_token(Token::RCurly),
            ',' => self.add_token(Token::Comma),
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Token::Slash);
                }
            }
            c if c.is_alphabetic() => self.identifier(),
            c if c.is_numeric() => self.numeric(),
            c => panic!("Unexpected character '{}'", c),
        };
    }

    fn string(&mut self) {
        let mut literal = String::new();

        while self.peek() != '"' {
            literal.push(self.advance());
        }
        self.advance();
        self.add_token(Token::String(literal));
    }

    fn numeric(&mut self) {
        let mut literal = String::new();
        literal.push(self.previous());

        while self.peek().is_numeric() || self.peek() == '.' {
            literal.push(self.advance());
        }

        self.add_token(Token::Numeric(
            literal.parse().expect("Error parsing number"),
        ));
    }

    fn identifier(&mut self) {
        let mut literal = String::new();
        literal.push(self.previous());

        while self.peek().is_alphanumeric() {
            literal.push(self.advance());
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

    fn advance(&mut self) -> char {
        let chr = self.peek();
        self.current += 1;
        chr
    }

    fn previous(&mut self) -> char {
        let chr = self.source.chars().nth(self.current - 1).unwrap();
        chr
    }

    fn peek(&mut self) -> char {
        let chr = self.source.chars().nth(self.current).unwrap();
        chr
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }

        if self.peek() != expected {
            return false;
        }

        true
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }
}
