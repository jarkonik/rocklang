use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    DoubleEqual,
    Else,
    Less,
    LessOrEqual,
    Comma,
    Break,
    True,
    False,
    NotEqual,
    Percent,
    LeftParen,
    LCurly,
    RCurly,
    While,
    If,
    RightParen,
    Slash,
    Plus,
    Minus,
    Asterisk,
    Equal,
    Arrow,
    Exclamation,
    Or,
    And,
    Greater,
    GreaterOrEqual,
    Colon,
    String(String),
    Identifier(String),
    Numeric(f64),
    Eof,
}

fn token_name(token: &Token) -> &str {
    match token {
        Token::NotEqual => "NotEqual",
        Token::LeftParen => "LeftParen",
        Token::RightParen => "RightParen",
        Token::Slash => "Slash",
        Token::Identifier { .. } => "Identifier",
        Token::Numeric { .. } => "Numeric",
        Token::Plus => "Plus",
        Token::Minus => "Minus",
        Token::Asterisk => "Asterisk",
        Token::Equal => "Equal",
        Token::LCurly => "LCurly",
        Token::RCurly => "RCurly",
        Token::If => "If",
        Token::While => "While",
        Token::True => "True",
        Token::False => "False",
        Token::DoubleEqual => "DoubleEqual",
        Token::Percent => "Percent",
        Token::Exclamation => "Exclamation",
        Token::Break => "Break",
        Token::String { .. } => "String",
        Token::Eof => "Eof",
        Token::Comma => "Comma",
        Token::Arrow => "Arrow",
        Token::Less => "Less",
        Token::LessOrEqual => "LessOrEqual",
        Token::Greater => "Greater",
        Token::GreaterOrEqual => "GreaterOrEqual",
        Token::Or => "Or",
        Token::And => "And",
        Token::Else => "Else",
        Token::Colon => "Colon",
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Token::Numeric(value) => {
                write!(f, "<{}({})>", token_name(self), value)
            }
            Token::Identifier(literal) => {
                write!(f, "<{}({})>", token_name(self), literal)
            }
            Token::String(literal) => {
                write!(f, "<{}({})>", token_name(self), literal)
            }
            _ => write!(f, "<{}>", token_name(self)),
        }
    }
}
