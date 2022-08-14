use std::fmt;
use std::fmt::Display;

use crate::parser::Span;

#[derive(Clone, Debug)]
pub enum TokenKind {
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
    F64(f64),
    Load,
    Extern,
    Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

fn token_name(token: &TokenKind) -> &str {
    match token {
        TokenKind::NotEqual => "NotEqual",
        TokenKind::LeftParen => "LeftParen",
        TokenKind::RightParen => "RightParen",
        TokenKind::Slash => "Slash",
        TokenKind::Identifier { .. } => "Identifier",
        TokenKind::F64 { .. } => "F64",
        TokenKind::Plus => "Plus",
        TokenKind::Minus => "Minus",
        TokenKind::Asterisk => "Asterisk",
        TokenKind::Equal => "Equal",
        TokenKind::LCurly => "LCurly",
        TokenKind::RCurly => "RCurly",
        TokenKind::If => "If",
        TokenKind::While => "While",
        TokenKind::True => "True",
        TokenKind::False => "False",
        TokenKind::DoubleEqual => "DoubleEqual",
        TokenKind::Percent => "Percent",
        TokenKind::Exclamation => "Exclamation",
        TokenKind::Break => "Break",
        TokenKind::String { .. } => "String",
        TokenKind::Eof => "Eof",
        TokenKind::Comma => "Comma",
        TokenKind::Arrow => "Arrow",
        TokenKind::Less => "Less",
        TokenKind::LessOrEqual => "LessOrEqual",
        TokenKind::Greater => "Greater",
        TokenKind::GreaterOrEqual => "GreaterOrEqual",
        TokenKind::Or => "Or",
        TokenKind::And => "And",
        TokenKind::Else => "Else",
        TokenKind::Colon => "Colon",
        TokenKind::Load => "Load",
        TokenKind::Extern => "Extern",
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TokenKind::F64(value) => {
                write!(f, "<{}({})>", token_name(self), value)
            }
            TokenKind::Identifier(literal) | TokenKind::String(literal) => {
                write!(f, "<{}({})>", token_name(self), literal)
            }
            _ => write!(f, "<{}>", token_name(self)),
        }
    }
}
