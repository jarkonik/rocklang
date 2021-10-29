use crate::expression;
use crate::expression::{Expression, Operator};
use crate::token::Token;
use serde::Serialize;
use std::error::Error;
use std::fmt;

#[derive(Clone)]
pub struct SyntaxError {
    token: Token,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TOKEN[{}]", self.token)
    }
}

impl fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax error: unexpected token {}", self.token)
    }
}

impl Error for SyntaxError {}

type Result<T> = std::result::Result<T, SyntaxError>;

#[derive(Clone, Serialize, Debug)]
pub enum Type {
    Numeric,
    Vector,
}

#[derive(Clone, Serialize, Debug)]
pub struct Param {
    pub typ: Type,
    pub name: String,
}

#[derive(Serialize, Clone)]
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
                        return Err(SyntaxError {
                            token: self.previous().clone(),
                        })
                    }
                };

                let mut body: Vec<Expression> = Vec::new();

                while match self.peek() {
                    Token::RCurly => {
                        self.advance();
                        false
                    }
                    _ => {
                        body.push(self.expression()?);
                        true
                    }
                } {}

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
                        return Err(SyntaxError {
                            token: self.previous().clone(),
                        })
                    }
                };

                let mut body: Vec<Expression> = Vec::new();
                let mut else_body: Vec<Expression> = Vec::new();

                while match self.peek() {
                    Token::RCurly => {
                        self.advance();
                        false
                    }
                    _ => {
                        body.push(self.expression()?);
                        true
                    }
                } {}

                if let Token::Else = self.peek() {
                    self.advance();

                    match self.advance() {
                        Token::LCurly => (),
                        _ => {
                            return Err(SyntaxError {
                                token: self.previous().clone(),
                            })
                        }
                    };

                    while match self.peek() {
                        Token::RCurly => {
                            self.advance();
                            false
                        }
                        _ => {
                            else_body.push(self.expression()?);
                            true
                        }
                    } {}
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

        while match self.peek() {
            Token::Equal => {
                self.advance();
                expr = Expression::Assignment(expression::Assignment {
                    left: Box::new(expr),
                    right: Box::new(self.equality()?),
                });
                true
            }
            _ => false,
        } {}

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression> {
        let mut expr = self.term()?;

        while match self.peek() {
            Token::DoubleEqual => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Equal,
                    right: Box::new(self.term()?),
                });
                true
            }
            Token::NotEqual => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::NotEqual,
                    right: Box::new(self.term()?),
                });
                true
            }
            Token::LessOrEqual => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::LessOrEqual,
                    right: Box::new(self.term()?),
                });
                true
            }
            Token::Less => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Less,
                    right: Box::new(self.term()?),
                });
                true
            }
            Token::Greater => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Greater,
                    right: Box::new(self.term()?),
                });
                true
            }
            _ => false,
        } {}

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression> {
        let mut expr = self.factor()?;

        while match self.peek() {
            Token::Plus => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Plus,
                    right: Box::new(self.factor()?),
                });
                true
            }
            Token::Minus => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Minus,
                    right: Box::new(self.factor()?),
                });
                true
            }
            Token::Percent => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Mod,
                    right: Box::new(self.factor()?),
                });
                true
            }
            _ => false,
        } {}

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression> {
        let mut expr = self.unary()?;

        while match self.peek() {
            Token::Asterisk => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Asterisk,
                    right: Box::new(self.unary()?),
                });
                true
            }
            Token::Slash => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Slash,
                    right: Box::new(self.unary()?),
                });
                true
            }
            _ => false,
        } {}

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
            _ => self.func_declr(),
        }
    }

    fn func_declr(&mut self) -> Result<Expression> {
        match self.peek() {
            Token::LeftParen => {
                let current = self.current;

                self.advance();

                let mut params: Vec<Param> = Vec::new();

                while match self.advance().clone() {
                    Token::Identifier(name_literal) => {
                        match self.advance() {
                            Token::Colon => match self.advance() {
                                Token::Identifier(type_literal) => {
                                    params.push(Param {
                                        name: name_literal.to_string(),
                                        typ: match type_literal.as_str() {
                                            "number" => Type::Numeric,
                                            "vec" => Type::Vector,
                                            _ => panic!("unkown type {}", type_literal),
                                        },
                                    });
                                }
                                _ => {
                                    return Err(SyntaxError {
                                        token: self.previous().clone(),
                                    })
                                }
                            },
                            _ => {
                                return Err(SyntaxError {
                                    token: self.previous().clone(),
                                })
                            }
                        }
                        true
                    }
                    Token::Comma => true,
                    Token::RightParen => false,
                    _ => {
                        self.current = current;
                        return self.func_call();
                    }
                } {}

                match self.advance() {
                    Token::Arrow => (),
                    _ => {
                        return Err(SyntaxError {
                            token: self.previous().clone(),
                        })
                    }
                }

                match self.advance() {
                    Token::LCurly => (),
                    _ => {
                        return Err(SyntaxError {
                            token: self.previous().clone(),
                        })
                    }
                }

                let mut body: Vec<Expression> = Vec::new();

                while match self.peek() {
                    Token::RCurly => {
                        self.advance();
                        false
                    }
                    _ => {
                        body.push(self.expression()?);
                        true
                    }
                } {}
                Ok(Expression::FuncDecl(expression::FuncDecl { body, params }))
            }
            _ => self.func_call(),
        }
    }

    fn func_call(&mut self) -> Result<Expression> {
        let mut expr = self.primary()?;

        while match self.peek() {
            Token::LeftParen => {
                match expr {
                    Expression::Identifier { .. } => {
                        self.advance();
                        let mut args: Vec<Expression> = Vec::new();

                        while match self.peek() {
                            Token::Comma => {
                                self.advance();
                                true
                            }
                            Token::RightParen => {
                                self.advance();
                                false
                            }
                            _ => {
                                args.push(self.expression()?);
                                true
                            }
                        } {}

                        expr = Expression::FuncCall(expression::FuncCall {
                            calee: Box::new(expr),
                            args,
                        });
                    }
                    _ => {
                        return Err(SyntaxError {
                            token: self.peek().clone(),
                        })
                    }
                }

                true
            }
            _ => false,
        } {}
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expression> {
        match self.advance() {
            Token::Numeric(val) => Ok(Expression::Numeric(*val)),
            Token::LeftParen => {
                let expr = Expression::Grouping(Box::new(self.expression()?));

                match self.advance() {
                    Token::RightParen => (),
                    _ => {
                        return Err(SyntaxError {
                            token: self.previous().clone(),
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
            _ => {
                return Err(SyntaxError {
                    token: self.previous().clone(),
                })
            }
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

    fn at_end(&mut self) -> bool {
        matches!(self.peek(), Token::Eof)
    }
}
