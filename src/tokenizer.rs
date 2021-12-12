use crate::token::Token;
use std::error::Error;
use std::fmt;

pub trait Tokenize {
    fn tokenize(&mut self) -> Result<&Vec<Token>>;
}

pub struct Tokenizer {
    source: String,
    current: usize,
    start: usize,
    tokens: Vec<Token>,
    line: usize,
}

impl Tokenize for Tokenizer {
    fn tokenize(&mut self) -> Result<&Vec<Token>> {
        self.current = 0;
        self.start = 0;
        self.line = 1;
        self.tokens.clear();

        while !self.at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::Eof);

        Ok(&self.tokens)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenizerError {
    pub chr: char,
    pub line: usize,
}

impl Error for TokenizerError {}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected character '{}'", self.chr)
    }
}

type Result<T> = std::result::Result<T, TokenizerError>;

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

    fn scan_token(&mut self) -> Result<()> {
        let chr = self.advance();
        match chr {
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '<' => {
                if '=' == self.advance() {
                    self.add_token(Token::LessOrEqual)
                } else {
                    self.add_token(Token::Less)
                }
            }
            '>' => {
                if '=' == self.advance() {
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
                if '=' == self.advance() {
                    self.add_token(Token::NotEqual);
                } else {
                    self.add_token(Token::Exclamation);
                }
            }
            '|' if self.advance() == '|' => {
                self.add_token(Token::Or);
            }
            '&' if self.advance() == '&' => {
                self.add_token(Token::And);
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
            ':' => self.add_token(Token::Colon),
            '/' => {
                if '/' == self.peek() {
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Token::Slash);
                }
            }
            c if c.is_alphabetic() => self.identifier(),
            c if c.is_numeric() => self.numeric(),
            chr => {
                return Err(TokenizerError {
                    chr,
                    line: self.line,
                })
            }
        };
        Ok(())
    }

    fn string(&mut self) {
        let mut literal = String::new();

        while self.peek() != '"' {
            let chr = self.advance();
            literal.push(chr);
        }
        self.advance();
        self.add_token(Token::String(literal));
    }

    fn numeric(&mut self) {
        let mut literal = String::new();
        literal.push(self.previous());

        loop {
            let chr = self.peek();

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

        loop {
            let chr = self.peek();

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
            "load" => self.add_token(Token::Load),
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
        self.source.chars().nth(self.current).unwrap()
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }
}
