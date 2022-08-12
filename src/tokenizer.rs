use crate::parser::Span;
use crate::token::{Token, TokenKind};
use std::error::Error;
use std::fmt;

pub trait Tokenize {
    fn tokenize(&mut self) -> Result<&Vec<Token>>;
}

pub struct Tokenizer {
    source: String,
    current: usize,
    column: usize,
    tokens: Vec<Token>,
    line: usize,
}

impl Tokenize for Tokenizer {
    fn tokenize(&mut self) -> Result<&Vec<Token>> {
        self.current = 0;
        self.line = 1;
        self.column = 1;
        self.tokens.clear();

        while !self.at_end() {
            self.scan_token()?;
        }

        self.add_token(TokenKind::Eof);

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
            column: 0,
            tokens: Vec::new(),
        }
    }

    fn scan_token(&mut self) -> Result<()> {
        let chr = self.advance();
        match chr {
            ' ' | '\r' | '\t' => (),
            '\n' => {
                self.line += 1;
                self.column = 0;
            }
            '<' => {
                if '=' == self.peek() {
                    self.advance();
                    self.add_token(TokenKind::LessOrEqual)
                } else {
                    self.add_token(TokenKind::Less)
                }
            }
            '>' => {
                if '=' == self.peek() {
                    self.advance();
                    self.add_token(TokenKind::GreaterOrEqual)
                } else {
                    self.add_token(TokenKind::Greater)
                }
            }
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '+' => self.add_token(TokenKind::Plus),
            '-' => self.add_token(TokenKind::Minus),
            '*' => self.add_token(TokenKind::Asterisk),
            '%' => self.add_token(TokenKind::Percent),
            '"' => self.string(),
            '!' => {
                if '=' == self.peek() {
                    self.advance();
                    self.add_token(TokenKind::NotEqual);
                } else {
                    self.add_token(TokenKind::Exclamation);
                }
            }
            '|' if self.advance() == '|' => {
                self.add_token(TokenKind::Or);
            }
            '&' if self.advance() == '&' => {
                self.add_token(TokenKind::And);
            }
            '=' => match self.peek() {
                '=' => {
                    self.add_token(TokenKind::DoubleEqual);
                    self.advance();
                }
                '>' => {
                    self.add_token(TokenKind::Arrow);
                    self.advance();
                }
                _ => self.add_token(TokenKind::Equal),
            },
            '{' => self.add_token(TokenKind::LCurly),
            '}' => self.add_token(TokenKind::RCurly),
            ',' => self.add_token(TokenKind::Comma),
            ':' => self.add_token(TokenKind::Colon),
            '/' => {
                if '/' == self.peek() {
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenKind::Slash);
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
        self.add_token(TokenKind::String(literal));
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

        self.add_token(TokenKind::Numeric(
            literal.parse().expect("Error parsing number"),
        ));
    }

    fn identifier(&mut self) {
        let mut literal = String::new();
        literal.push(self.previous());

        loop {
            let chr = self.peek();

            if chr.is_alphanumeric() || chr == '_' {
                literal.push(chr);
                self.advance();
            } else {
                break;
            }
        }

        match literal.as_str() {
            "if" => self.add_token(TokenKind::If),
            "while" => self.add_token(TokenKind::While),
            "true" => self.add_token(TokenKind::True),
            "false" => self.add_token(TokenKind::False),
            "break" => self.add_token(TokenKind::Break),
            "else" => self.add_token(TokenKind::Else),
            "load" => self.add_token(TokenKind::Load),
            "extern" => self.add_token(TokenKind::Extern),
            _ => self.add_token(TokenKind::Identifier(literal)),
        };
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token {
            kind,
            span: Span {
                line: self.line as u32,
                column: self.column as u32,
            },
        });
    }

    fn advance(&mut self) -> char {
        let chr = self.peek();
        self.column += 1;
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
