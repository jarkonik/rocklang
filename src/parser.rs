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

impl PartialEq for ParserError {
    fn eq(&self, rhs: &ParserError) -> bool {
        match (self, rhs) {
            (
                ParserError::SyntaxError {
                    token: a,
                    backtrace: _,
                },
                ParserError::SyntaxError {
                    token: b,
                    backtrace: _,
                },
            ) => a == b,
        }
    }
}

type Result<T> = std::result::Result<T, ParserError>;

#[derive(Copy, Clone, Serialize, Debug)]
pub enum Type {
    Numeric,
    Vector,
    Null,
    Function,
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
                        return Err(ParserError::SyntaxError {
                            token: self.previous().clone(),
                            backtrace: Backtrace::new(),
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
                        return Err(ParserError::SyntaxError {
                            token: self.previous().clone(),
                            backtrace: Backtrace::new(),
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
                            return Err(ParserError::SyntaxError {
                                token: self.previous().clone(),
                                backtrace: Backtrace::new(),
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
        let mut expr = self.addition_or_modulo()?;

        while match self.peek() {
            Token::DoubleEqual => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Equal,
                    right: Box::new(self.addition_or_modulo()?),
                });
                true
            }
            Token::NotEqual => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::NotEqual,
                    right: Box::new(self.addition_or_modulo()?),
                });
                true
            }
            Token::LessOrEqual => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::LessOrEqual,
                    right: Box::new(self.addition_or_modulo()?),
                });
                true
            }
            Token::Less => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Less,
                    right: Box::new(self.addition_or_modulo()?),
                });
                true
            }
            Token::Greater => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Greater,
                    right: Box::new(self.addition_or_modulo()?),
                });
                true
            }
            Token::GreaterOrEqual => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::GreaterOrEqual,
                    right: Box::new(self.addition_or_modulo()?),
                });
                true
            }
            _ => false,
        } {}

        Ok(expr)
    }

    fn addition_or_modulo(&mut self) -> Result<Expression> {
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
                                            "fun" => Type::Function,
                                            _ => {
                                                return Err(ParserError::SyntaxError {
                                                    token: self.previous().clone(),
                                                    backtrace: Backtrace::new(),
                                                })
                                            }
                                        },
                                    });
                                }
                                _ => {
                                    return Err(ParserError::SyntaxError {
                                        token: self.previous().clone(),
                                        backtrace: Backtrace::new(),
                                    })
                                }
                            },
                            _ => {
                                self.current = current;
                                return self.func_call();
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
                        "void" => Type::Null,
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
        let mut expr = self.import()?;

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
                        return Err(ParserError::SyntaxError {
                            token: self.peek().clone(),
                            backtrace: Backtrace::new(),
                        })
                    }
                }

                true
            }
            _ => false,
        } {}
        Ok(expr)
    }

    fn import(&mut self) -> Result<Expression> {
        match self.peek() {
            Token::Load => {
                self.advance();
                match self.advance() {
                    Token::String(s) => Ok(Expression::Load(s.to_string())),
                    _ => Err(ParserError::SyntaxError {
                        token: self.previous().clone(),
                        backtrace: Backtrace::new(),
                    }),
                }
            }
            _ => self.extern_stmt(),
        }
    }

    fn extern_stmt(&mut self) -> Result<Expression> {
        match self.peek() {
            Token::Extern => {
                self.advance();
                match self.advance() {
                    Token::String(s) => Ok(Expression::Extern(self.func_declr()?)),
                    _ => Err(ParserError::SyntaxError {
                        token: self.previous().clone(),
                        backtrace: Backtrace::new(),
                    }),
                }
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

    fn at_end(&mut self) -> bool {
        matches!(self.peek(), Token::Eof)
    }
}
