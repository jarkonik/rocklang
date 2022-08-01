use crate::expression;
use crate::expression::{Expression, Operator};
use crate::token::Token;
use backtrace::Backtrace;
use serde::Serialize;
use std::error::Error;
use std::fmt::Display;

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
                write!(f, "Syntax error: unexpected token {}", token)
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
}

#[derive(Clone, Serialize, Debug)]
pub struct Param {
    pub typ: Type,
    pub name: String,
}

#[derive(Default, Serialize, Clone)]
pub struct Program {
    pub body: Vec<Expression>,
}

pub trait Parse {
    fn parse(&mut self) -> Result<Program>;
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parse for Parser {
    fn parse(&mut self) -> Result<Program> {
        let mut statements: Vec<Expression> = Vec::new();

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

    fn expression(&mut self) -> Result<Expression> {
        self.while_loop()
    }

    fn while_loop(&mut self) -> Result<Expression> {
        match self.peek() {
            Token::While => {
                self.advance();
                let predicate = self.expression()?;

                match self.advance() {
                    Token::LCurly => (),
                    _ => {
                        return Err(ParserError::SyntaxError {
                            token: self.previous().clone(),
                            backtrace: Backtrace::new(),
                        })
                    }
                };

                let mut body: Vec<Expression> = Vec::new();

                loop {
                    match self.peek() {
                        Token::RCurly => {
                            self.advance();
                            break;
                        }
                        _ => {
                            body.push(self.expression()?);
                        }
                    }
                }

                Ok(Expression::While(expression::While {
                    predicate: Box::new(predicate),
                    body,
                }))
            }
            _ => self.conditional(),
        }
    }

    fn conditional(&mut self) -> Result<Expression> {
        match self.peek() {
            Token::If => {
                self.advance();
                let predicate = self.expression()?;

                match self.advance() {
                    Token::LCurly => (),
                    _ => {
                        return Err(ParserError::SyntaxError {
                            token: self.previous().clone(),
                            backtrace: Backtrace::new(),
                        })
                    }
                };

                let mut body: Vec<Expression> = Vec::new();
                let mut else_body: Vec<Expression> = Vec::new();

                loop {
                    match self.peek() {
                        Token::RCurly => {
                            self.advance();
                            break;
                        }
                        _ => {
                            body.push(self.expression()?);
                        }
                    }
                }

                if let Token::Else = self.peek() {
                    self.advance();

                    match self.advance() {
                        Token::LCurly => (),
                        _ => {
                            return Err(ParserError::SyntaxError {
                                token: self.previous().clone(),
                                backtrace: Backtrace::new(),
                            })
                        }
                    };

                    loop {
                        match self.peek() {
                            Token::RCurly => {
                                self.advance();
                                break;
                            }
                            _ => {
                                else_body.push(self.expression()?);
                            }
                        }
                    }
                }

                Ok(Expression::Conditional(expression::Conditional {
                    predicate: Box::new(predicate),
                    body,
                    else_body,
                }))
            }
            _ => self.assignment(),
        }
    }

    fn assignment(&mut self) -> Result<Expression> {
        let mut expr = self.equality()?;

        while let Token::Equal = self.peek() {
            self.advance();
            expr = Expression::Assignment(expression::Assignment {
                left: Box::new(expr),
                right: Box::new(self.equality()?),
            });
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression> {
        let mut expr = self.addition_or_modulo()?;

        loop {
            match self.peek() {
                Token::DoubleEqual => {
                    self.advance();
                    expr = Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Equal,
                        right: Box::new(self.addition_or_modulo()?),
                    });
                }
                Token::NotEqual => {
                    self.advance();
                    expr = Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::NotEqual,
                        right: Box::new(self.addition_or_modulo()?),
                    });
                }
                Token::LessOrEqual => {
                    self.advance();
                    expr = Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::LessOrEqual,
                        right: Box::new(self.addition_or_modulo()?),
                    });
                }
                Token::Less => {
                    self.advance();
                    expr = Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Less,
                        right: Box::new(self.addition_or_modulo()?),
                    });
                }
                Token::Greater => {
                    self.advance();
                    expr = Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Greater,
                        right: Box::new(self.addition_or_modulo()?),
                    });
                }
                Token::GreaterOrEqual => {
                    self.advance();
                    expr = Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::GreaterOrEqual,
                        right: Box::new(self.addition_or_modulo()?),
                    });
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn addition_or_modulo(&mut self) -> Result<Expression> {
        let mut expr = self.factor()?;

        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance();
                    expr = Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Plus,
                        right: Box::new(self.factor()?),
                    });
                }
                Token::Minus => {
                    self.advance();
                    expr = Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Minus,
                        right: Box::new(self.factor()?),
                    });
                }
                Token::Percent => {
                    self.advance();
                    expr = Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Mod,
                        right: Box::new(self.factor()?),
                    });
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression> {
        let mut expr = self.unary()?;

        loop {
            match self.peek() {
                Token::Asterisk => {
                    self.advance();
                    expr = Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Asterisk,
                        right: Box::new(self.unary()?),
                    });
                }
                Token::Slash => {
                    self.advance();
                    expr = Expression::Binary(expression::Binary {
                        left: Box::new(expr),
                        operator: Operator::Slash,
                        right: Box::new(self.unary()?),
                    });
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression> {
        match self.peek() {
            Token::Minus => {
                self.advance();
                Ok(Expression::Unary(expression::Unary {
                    operator: Operator::Minus,
                    right: Box::new(self.unary()?),
                }))
            }
            _ => self.extern_stmt(),
        }
    }

    fn extern_stmt(&mut self) -> Result<Expression> {
        match self.peek() {
            Token::Extern => {
                self.advance();

                if !matches!(self.advance(), Token::Less) {
                    return Err(ParserError::SyntaxError {
                        token: self.previous().clone(),
                        backtrace: Backtrace::new(),
                    });
                };

                let mut types = vec![];

                loop {
                    let token = self.advance().clone();
                    match token {
                        Token::Identifier(ref s) => {
                            types.push(self.type_from_literal(s)?);
                        }
                        Token::Comma => (),
                        Token::Greater => {
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

                if !matches!(self.advance(), Token::LeftParen) {
                    return Err(ParserError::SyntaxError {
                        token: self.previous().clone(),
                        backtrace: Backtrace::new(),
                    });
                };

                let name = if let Token::String(s) = self.advance() {
                    s.to_string()
                } else {
                    return Err(ParserError::SyntaxError {
                        token: self.previous().clone(),
                        backtrace: Backtrace::new(),
                    });
                };

                if !matches!(self.advance(), Token::RightParen) {
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

                Ok(Expression::Extern(expression::Extern {
                    types: types[0..types.len() - 1].to_vec(),
                    return_type,
                    name,
                }))
            }
            _ => self.func_declr(),
        }
    }

    fn func_declr(&mut self) -> Result<Expression> {
        match self.peek() {
            Token::LeftParen => {
                let current = self.current;

                self.advance();

                let mut params: Vec<Param> = Vec::new();

                loop {
                    match self.advance().clone() {
                        Token::Identifier(name_literal) => match self.advance() {
                            Token::Colon => {
                                let token = self.advance().clone();
                                match token {
                                    Token::Identifier(type_literal) => {
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
                        Token::Comma => {}
                        Token::RightParen => break,
                        _ => {
                            self.current = current;
                            return self.func_call();
                        }
                    }
                }

                match self.advance() {
                    Token::Colon => (),
                    _ => {
                        return Err(ParserError::SyntaxError {
                            token: self.previous().clone(),
                            backtrace: Backtrace::new(),
                        })
                    }
                }

                let return_type = match self.advance() {
                    Token::Identifier(type_literal) => match type_literal.as_str() {
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

                match self.advance() {
                    Token::Arrow => (),
                    _ => {
                        return Err(ParserError::SyntaxError {
                            token: self.previous().clone(),
                            backtrace: Backtrace::new(),
                        })
                    }
                }

                match self.advance() {
                    Token::LCurly => (),
                    _ => {
                        return Err(ParserError::SyntaxError {
                            token: self.previous().clone(),
                            backtrace: Backtrace::new(),
                        })
                    }
                }

                let mut body: Vec<Expression> = Vec::new();

                loop {
                    match self.peek() {
                        Token::RCurly => {
                            self.advance();
                            break;
                        }
                        _ => {
                            body.push(self.expression()?);
                        }
                    }
                }
                Ok(Expression::FuncDecl(expression::FuncDecl {
                    body,
                    params,

                    return_type,
                }))
            }
            _ => self.func_call(),
        }
    }

    fn func_call(&mut self) -> Result<Expression> {
        let mut expr = self.load()?;

        while let Token::LeftParen = self.peek() {
            match expr {
                Expression::Identifier { .. } => {
                    self.advance();
                    let mut args: Vec<Expression> = Vec::new();

                    loop {
                        match self.peek() {
                            Token::Comma => {
                                self.advance();
                            }
                            Token::RightParen => {
                                self.advance();
                                break;
                            }
                            _ => {
                                args.push(self.expression()?);
                            }
                        }
                    }

                    expr = Expression::FuncCall(expression::FuncCall {
                        calee: Box::new(expr),
                        args,
                    });
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

    fn load(&mut self) -> Result<Expression> {
        match self.peek() {
            Token::Load => {
                self.advance();

                if !matches!(self.advance(), Token::LeftParen) {
                    return Err(ParserError::SyntaxError {
                        token: self.previous().clone(),
                        backtrace: Backtrace::new(),
                    });
                };

                let name = if let Token::String(s) = self.advance() {
                    s.to_string()
                } else {
                    return Err(ParserError::SyntaxError {
                        token: self.previous().clone(),
                        backtrace: Backtrace::new(),
                    });
                };

                if !matches!(self.advance(), Token::RightParen) {
                    return Err(ParserError::SyntaxError {
                        token: self.previous().clone(),
                        backtrace: Backtrace::new(),
                    });
                };

                Ok(Expression::Load(name))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expression> {
        match self.advance() {
            Token::Numeric(val) => Ok(Expression::Numeric(*val)),
            Token::LeftParen => {
                let expr = Expression::Grouping(Box::new(self.expression()?));

                match self.advance() {
                    Token::RightParen => (),
                    token => {
                        return Err(ParserError::SyntaxError {
                            token: token.clone(),
                            backtrace: Backtrace::new(),
                        })
                    }
                };

                Ok(expr)
            }
            Token::Identifier(literal) => Ok(Expression::Identifier(literal.to_string())),
            Token::String(literal) => Ok(Expression::String(literal.to_string())),
            Token::True => Ok(Expression::Bool(true)),
            Token::False => Ok(Expression::Bool(false)),
            Token::Break => Ok(Expression::Break),
            token => Err(ParserError::SyntaxError {
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
        matches!(self.peek(), Token::Eof)
    }
}
