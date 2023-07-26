use crate::expression::{self, Node};
use crate::expression::{Expression, Operator};
use crate::token::{Token, TokenKind};
use backtrace::Backtrace;
use serde::Serialize;
use std::error::Error;
use std::fmt::Display;

macro_rules! consume {
    ($self: ident,$kind: pat) => {{
        let token = $self.advance();
        if matches!(token.kind, $kind) {
            Ok(())
        } else {
            Err(ParserError::SyntaxError {
                token: token.clone(),
                backtrace: Backtrace::new(),
            })
        }
    }};
}

#[derive(Clone, Debug)]
pub enum ParserError {
    SyntaxError { token: Token, backtrace: Backtrace },
}
impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ParserError::SyntaxError {
                token,
                backtrace: _,
            } => {
                write!(
                    f,
                    "Syntax error: unexpected token {} at {}",
                    token.kind, token.span
                )
            }
        }
    }
}
impl Error for ParserError {}

type Result<T> = std::result::Result<T, ParserError>;

#[derive(Copy, Clone, Serialize, Debug)]
pub enum Type {
    Numeric,
    Bool,
    Vector,
    Void,
    Function,
    Ptr,
    String,
    CString,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Type::Numeric => "Numeric",
            Type::Bool => "Bool",
            Type::Vector => "Vector",
            Type::Void => "Void",
            Type::Function => "Function",
            Type::Ptr => "Ptr",
            Type::String => "String",
            Type::CString => "CString",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct Param {
    pub typ: Type,
    pub name: String,
}

#[derive(Default, Serialize, Clone)]
pub struct Program {
    pub body: Vec<Node>,
}

pub trait Parse {
    fn parse(&mut self) -> Result<Program>;
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug, Default, Serialize, Clone)]
pub struct Span {
    pub line: u32,
    pub column: u32,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Parse for Parser {
    fn parse(&mut self) -> Result<Program> {
        let mut statements: Vec<Node> = Vec::new();

        while !self.at_end() {
            statements.push(self.expression()?);
        }

        Ok(Program { body: statements })
    }
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Parser {
            tokens: tokens.to_vec(),
            current: 0,
        }
    }

    fn expression(&mut self) -> Result<Node> {
        self.while_loop()
    }

    fn while_loop(&mut self) -> Result<Node> {
        match self.peek().kind {
            TokenKind::While => {
                self.advance();
                let predicate = self.expression()?;

                match self.advance().kind {
                    TokenKind::LCurly => (),
                    _ => {
                        return Err(ParserError::SyntaxError {
                            token: self.previous().clone(),
                            backtrace: Backtrace::new(),
                        })
                    }
                };

                let mut body: Vec<Node> = Vec::new();

                loop {
                    match self.peek().kind {
                        TokenKind::RCurly => {
                            self.advance();
                            break;
                        }
                        _ => {
                            body.push(self.expression()?);
                        }
                    }
                }

                Ok(self.node(Expression::While(expression::While {
                    predicate: Box::new(predicate),
                    body,
                })))
            }
            _ => self.conditional(),
        }
    }

    fn node(&mut self, expression: Expression) -> Node {
        Node {
            expression,
            span: self.previous().span.clone(),
        }
    }

    fn conditional(&mut self) -> Result<Node> {
        match self.peek().kind {
            TokenKind::If => {
                self.advance();
                let predicate = self.expression()?;

                match self.advance().kind {
                    TokenKind::LCurly => (),
                    _ => {
                        return Err(ParserError::SyntaxError {
                            token: self.previous().clone(),
                            backtrace: Backtrace::new(),
                        })
                    }
                };

                let mut body: Vec<Node> = Vec::new();
                let mut else_body: Vec<Node> = Vec::new();

                loop {
                    match self.peek().kind {
                        TokenKind::RCurly => {
                            self.advance();
                            break;
                        }
                        _ => {
                            body.push(self.expression()?);
                        }
                    }
                }

                if let TokenKind::Else = self.peek().kind {
                    self.advance();

                    match self.advance().kind {
                        TokenKind::LCurly => (),
                        _ => {
                            return Err(ParserError::SyntaxError {
                                token: self.previous().clone(),
                                backtrace: Backtrace::new(),
                            })
                        }
                    };

                    loop {
                        match self.peek().kind {
                            TokenKind::RCurly => {
                                self.advance();
                                break;
                            }
                            _ => {
                                else_body.push(self.expression()?);
                            }
                        }
                    }
                }

                Ok(self.node(Expression::Conditional(expression::Conditional {
                    predicate: Box::new(predicate),
                    body,
                    else_body,
                })))
            }
            _ => self.assignment(),
        }
    }

    fn assignment(&mut self) -> Result<Node> {
        let mut expr = self.equality()?;

        while let TokenKind::Equal = self.peek().kind {
            self.advance();
            let right = self.equality()?;
            expr = self.node(Expression::Assignment(expression::Assignment {
                left: Box::new(expr),
                right: Box::new(right),
            }))
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Node> {
        let mut expr = self.addition_or_modulo()?;

        loop {
            match self.peek().kind {
                TokenKind::DoubleEqual => {
                    self.advance();
                    let right = self.addition_or_modulo()?;
                    expr = self.node(Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Equal,
                        right: Box::new(right),
                    }));
                }
                TokenKind::NotEqual => {
                    self.advance();
                    let right = self.addition_or_modulo()?;
                    expr = self.node(Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::NotEqual,
                        right: Box::new(right),
                    }));
                }
                TokenKind::LessOrEqual => {
                    self.advance();
                    let right = self.addition_or_modulo()?;
                    expr = self.node(Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::LessOrEqual,
                        right: Box::new(right),
                    }));
                }
                TokenKind::Less => {
                    self.advance();
                    let right = self.addition_or_modulo()?;
                    expr = self.node(Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Less,
                        right: Box::new(right),
                    }));
                }
                TokenKind::Greater => {
                    self.advance();
                    let right = self.addition_or_modulo()?;
                    expr = self.node(Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Greater,
                        right: Box::new(right),
                    }));
                }
                TokenKind::GreaterOrEqual => {
                    self.advance();
                    let right = self.addition_or_modulo()?;
                    expr = self.node(Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::GreaterOrEqual,
                        right: Box::new(right),
                    }));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn addition_or_modulo(&mut self) -> Result<Node> {
        let mut expr = self.factor()?;

        loop {
            match self.peek().kind {
                TokenKind::Plus => {
                    self.advance();
                    let right = self.factor()?;

                    expr = self.node(Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Plus,
                        right: Box::new(right),
                    }));
                }
                TokenKind::Minus => {
                    self.advance();
                    let right = self.factor()?;
                    expr = self.node(Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Minus,
                        right: Box::new(right),
                    }));
                }
                TokenKind::Percent => {
                    self.advance();
                    let right = self.factor()?;
                    expr = self.node(Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Mod,
                        right: Box::new(right),
                    }));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Node> {
        let mut expr = self.unary()?;

        loop {
            match self.peek().kind {
                TokenKind::Asterisk => {
                    self.advance();
                    let right = self.unary()?;
                    expr = self.node(Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Asterisk,
                        right: Box::new(right),
                    }));
                }
                TokenKind::Slash => {
                    self.advance();
                    let right = self.unary()?;
                    expr = self.node(Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Slash,
                        right: Box::new(right),
                    }));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Node> {
        match self.peek().kind {
            TokenKind::Minus => {
                self.advance();
                let right = self.unary()?;
                Ok(self.node(Expression::Unary(expression::Unary {
                    operator: Operator::Minus,
                    right: Box::new(right),
                })))
            }
            _ => self.extern_stmt(),
        }
    }

    fn extern_stmt(&mut self) -> Result<Node> {
        match self.peek().kind {
            TokenKind::Extern => {
                self.advance();

                if !matches!(self.advance().kind, TokenKind::Less) {
                    return Err(ParserError::SyntaxError {
                        token: self.previous().clone(),
                        backtrace: Backtrace::new(),
                    });
                };

                let mut types = vec![];

                loop {
                    let token = self.advance().clone();
                    match token.kind {
                        TokenKind::Identifier(ref s) => {
                            types.push(self.type_from_literal(s)?);
                        }
                        TokenKind::Comma => (),
                        TokenKind::Greater => {
                            break;
                        }
                        _ => {
                            return Err(ParserError::SyntaxError {
                                token: self.previous().clone(),
                                backtrace: Backtrace::new(),
                            });
                        }
                    }
                }

                if !matches!(self.advance().kind, TokenKind::LeftParen) {
                    return Err(ParserError::SyntaxError {
                        token: self.previous().clone(),
                        backtrace: Backtrace::new(),
                    });
                };

                let name = if let TokenKind::String(s) = &self.advance().kind {
                    s.to_string()
                } else {
                    return Err(ParserError::SyntaxError {
                        token: self.previous().clone(),
                        backtrace: Backtrace::new(),
                    });
                };

                if !matches!(self.advance().kind, TokenKind::RightParen) {
                    return Err(ParserError::SyntaxError {
                        token: self.previous().clone(),
                        backtrace: Backtrace::new(),
                    });
                };

                let return_type = match types.last() {
                    Some(v) => *v,
                    _ => {
                        return Err(ParserError::SyntaxError {
                            token: self.previous().clone(),
                            backtrace: Backtrace::new(),
                        });
                    }
                };

                Ok(self.node(Expression::Extern(expression::Extern {
                    types: types[0..types.len() - 1].to_vec(),
                    return_type,
                    name,
                })))
            }
            _ => self.func_declr(),
        }
    }

    fn func_declr(&mut self) -> Result<Node> {
        match self.peek().kind {
            TokenKind::LeftParen => {
                let current = self.current;

                self.advance();

                let mut params: Vec<Param> = Vec::new();

                loop {
                    match self.advance().clone().kind {
                        TokenKind::Identifier(name_literal) => match self.advance().kind {
                            TokenKind::Colon => {
                                let token = self.advance().clone();
                                match token.kind {
                                    TokenKind::Identifier(type_literal) => {
                                        params.push(Param {
                                            name: name_literal.to_string(),
                                            typ: self.type_from_literal(&type_literal)?,
                                        });
                                    }
                                    _ => {
                                        return Err(ParserError::SyntaxError {
                                            token: self.previous().clone(),
                                            backtrace: Backtrace::new(),
                                        })
                                    }
                                }
                            }
                            _ => {
                                self.current = current;
                                return self.func_call();
                            }
                        },
                        TokenKind::Comma => {}
                        TokenKind::RightParen => break,
                        _ => {
                            self.current = current;
                            return self.func_call();
                        }
                    }
                }

                match self.advance().kind {
                    TokenKind::Arrow => (),
                    _ => {
                        return Err(ParserError::SyntaxError {
                            token: self.previous().clone(),
                            backtrace: Backtrace::new(),
                        })
                    }
                }

                let return_type = match &self.advance().kind {
                    TokenKind::Identifier(type_literal) => match type_literal.as_str() {
                        "number" => Type::Numeric,
                        "vec" => Type::Vector,
                        "void" => Type::Void,
                        _ => {
                            return Err(ParserError::SyntaxError {
                                token: self.previous().clone(),
                                backtrace: Backtrace::new(),
                            })
                        }
                    },
                    _ => {
                        return Err(ParserError::SyntaxError {
                            token: self.previous().clone(),
                            backtrace: Backtrace::new(),
                        })
                    }
                };


                match self.advance().kind {
                    TokenKind::LCurly => (),
                    _ => {
                        return Err(ParserError::SyntaxError {
                            token: self.previous().clone(),
                            backtrace: Backtrace::new(),
                        })
                    }
                }

                let mut body: Vec<Node> = Vec::new();

                loop {
                    match self.peek().kind {
                        TokenKind::RCurly => {
                            self.advance();
                            break;
                        }
                        _ => {
                            body.push(self.expression()?);
                        }
                    }
                }
                Ok(self.node(Expression::FuncDecl(expression::FuncDecl {
                    body,
                    params,

                    return_type,
                })))
            }
            _ => self.func_call(),
        }
    }

    fn func_call(&mut self) -> Result<Node> {
        let mut expr = self.load()?;

        while let TokenKind::LeftParen = self.peek().kind {
            match expr.expression {
                Expression::Identifier { .. } => {
                    self.advance();
                    let mut args: Vec<Node> = Vec::new();

                    loop {
                        if let TokenKind::RightParen = self.peek().kind {
                            self.advance();
                            break;
                        }
                        args.push(self.expression()?);
                        if let TokenKind::RightParen = self.peek().kind {
                            self.advance();
                            break;
                        }
                        consume!(self, TokenKind::Comma)?;
                    }

                    expr = self.node(Expression::FuncCall(expression::FuncCall {
                        calee: Box::new(expr),
                        args,
                    }));
                }
                _ => {
                    return Err(ParserError::SyntaxError {
                        token: self.peek().clone(),
                        backtrace: Backtrace::new(),
                    })
                }
            };
        }
        Ok(expr)
    }

    fn load(&mut self) -> Result<Node> {
        match self.peek().kind {
            TokenKind::Load => {
                self.advance();

                if !matches!(self.advance().kind, TokenKind::LeftParen) {
                    return Err(ParserError::SyntaxError {
                        token: self.previous().clone(),
                        backtrace: Backtrace::new(),
                    });
                };

                let name = if let TokenKind::String(s) = &self.advance().kind {
                    s.to_string()
                } else {
                    return Err(ParserError::SyntaxError {
                        token: self.previous().clone(),
                        backtrace: Backtrace::new(),
                    });
                };

                if !matches!(self.advance().kind, TokenKind::RightParen) {
                    return Err(ParserError::SyntaxError {
                        token: self.previous().clone(),
                        backtrace: Backtrace::new(),
                    });
                };

                Ok(self.node(Expression::Load(name)))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Node> {
        let token = self.advance().clone();
        match &token.kind {
            TokenKind::Numeric(val) => Ok(self.node(Expression::Numeric(*val))),
            TokenKind::LeftParen => {
                let expr = Expression::Grouping(expression::Grouping(Box::new(self.expression()?)));

                let token = self.advance();

                match token.kind {
                    TokenKind::RightParen => (),
                    _ => {
                        return Err(ParserError::SyntaxError {
                            token: token.clone(),
                            backtrace: Backtrace::new(),
                        })
                    }
                };

                Ok(self.node(expr))
            }
            TokenKind::Identifier(literal) => {
                Ok(self.node(Expression::Identifier(literal.to_string())))
            }
            TokenKind::String(literal) => Ok(self.node(Expression::String(literal.to_string()))),
            TokenKind::True => Ok(self.node(Expression::Bool(true))),
            TokenKind::False => Ok(self.node(Expression::Bool(false))),
            TokenKind::Break => Ok(self.node(Expression::Break)),
            _ => Err(ParserError::SyntaxError {
                token: token.clone(),
                backtrace: Backtrace::new(),
            }),
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn type_from_literal(&mut self, type_literal: &str) -> Result<Type> {
        match type_literal {
            "void" => Ok(Type::Void),
            "string" => Ok(Type::String),
            "cstring" => Ok(Type::CString),
            "number" => Ok(Type::Numeric),
            "vec" => Ok(Type::Vector),
            "fun" => Ok(Type::Function),
            "ptr" => Ok(Type::Ptr),
            _ => Err(ParserError::SyntaxError {
                token: self.previous().clone(),
                backtrace: Backtrace::new(),
            }),
        }
    }

    fn at_end(&mut self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! token {
        ($expr: expr) => {
            Token {
                kind: $expr,
                span: Span::default(),
            }
        };
    }

    macro_rules! assert_is_err {
        ($val: expr) => {
            assert!(matches!($val, Err(_)));
        };
    }

    #[test]
    fn dont_allow_args_without_commas_in_between() {
        let mut parser = Parser::new(&[
            token!(TokenKind::Identifier("test".to_string())),
            token!(TokenKind::LeftParen),
            token!(TokenKind::Identifier("a".to_string())),
            token!(TokenKind::Identifier("b".to_string())),
            token!(TokenKind::RightParen),
        ]);

        assert_is_err!(parser.parse());
    }
}
