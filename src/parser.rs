use crate::expression;
use crate::expression::{Expression, Operator};
use crate::token::Token;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Program {
    pub body: Vec<Expression>,
}

pub trait Parse {
    fn parse(&mut self) -> Program;
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parse for Parser {
    fn parse(&mut self) -> Program {
        let mut statements: Vec<Expression> = Vec::new();

        while !self.at_end() {
            statements.push(self.expression());
        }

        Program { body: statements }
    }
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Parser {
            tokens: tokens.to_vec(),
            current: 0,
        }
    }

    fn expression(&mut self) -> Expression {
        self.while_loop()
    }

    fn while_loop(&mut self) -> Expression {
        match self.peek() {
            Token::While => {
                self.advance();
                let predicate = self.expression();

                match self.advance() {
                    Token::LCurly => (),
                    _ => panic!("unexpected token {}", self.previous()),
                };

                let mut body: Vec<Expression> = Vec::new();

                while match self.peek() {
                    Token::RCurly => {
                        self.advance();
                        false
                    }
                    _ => {
                        body.push(self.expression());
                        true
                    }
                } {}

                Expression::While(expression::While {
                    predicate: Box::new(predicate),
                    body,
                })
            }
            _ => self.conditional(),
        }
    }

    fn conditional(&mut self) -> Expression {
        match self.peek() {
            Token::If => {
                self.advance();
                let predicate = self.expression();

                match self.advance() {
                    Token::LCurly => (),
                    _ => panic!("unexpected token {}", self.previous()),
                };

                let mut body: Vec<Expression> = Vec::new();
                let mut else_body: Vec<Expression> = Vec::new();

                while match self.peek() {
                    Token::RCurly => {
                        self.advance();
                        false
                    }
                    _ => {
                        body.push(self.expression());
                        true
                    }
                } {}

                if let Token::Else = self.peek() {
                    self.advance();

                    match self.advance() {
                        Token::LCurly => (),
                        _ => panic!("unexpected token {}", self.previous()),
                    };

                    while match self.peek() {
                        Token::RCurly => {
                            self.advance();
                            false
                        }
                        _ => {
                            else_body.push(self.expression());
                            true
                        }
                    } {}
                }

                Expression::Conditional(expression::Conditional {
                    predicate: Box::new(predicate),
                    body,
                    else_body,
                })
            }
            _ => self.assignment(),
        }
    }

    fn assignment(&mut self) -> Expression {
        let mut expr = self.equality();

        while match self.peek() {
            Token::Equal => {
                self.advance();
                expr = Expression::Assignment(expression::Assignment {
                    left: Box::new(expr),
                    right: Box::new(self.equality()),
                });
                true
            }
            _ => false,
        } {}

        expr
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.term();

        while match self.peek() {
            Token::DoubleEqual => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Equal,
                    right: Box::new(self.term()),
                });
                true
            }
            Token::NotEqual => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::NotEqual,
                    right: Box::new(self.term()),
                });
                true
            }
            Token::LessOrEqual => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::LessOrEqual,
                    right: Box::new(self.term()),
                });
                true
            }
            Token::Less => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Less,
                    right: Box::new(self.term()),
                });
                true
            }
            _ => false,
        } {}

        expr
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        while match self.peek() {
            Token::Plus => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Plus,
                    right: Box::new(self.factor()),
                });
                true
            }
            Token::Minus => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Minus,
                    right: Box::new(self.factor()),
                });
                true
            }
            Token::Percent => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Mod,
                    right: Box::new(self.factor()),
                });
                true
            }
            _ => false,
        } {}

        expr
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        while match self.peek() {
            Token::Asterisk => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Asterisk,
                    right: Box::new(self.unary()),
                });
                true
            }
            Token::Slash => {
                self.advance();
                expr = Expression::Binary(expression::Binary {
                    left: Box::new(expr),
                    operator: Operator::Slash,
                    right: Box::new(self.unary()),
                });
                true
            }
            _ => false,
        } {}

        expr
    }

    fn unary(&mut self) -> Expression {
        match self.peek() {
            Token::Minus => {
                self.advance();
                Expression::Unary(expression::Unary {
                    operator: Operator::Minus,
                    right: Box::new(self.unary()),
                })
            }
            _ => self.func_declr(),
        }
    }

    fn func_declr(&mut self) -> Expression {
        match self.peek() {
            Token::LeftParen => {
                self.advance();

                let mut params: Vec<String> = Vec::new();

                while match self.advance() {
                    Token::Identifier(literal) => {
                        params.push(literal.to_string());
                        true
                    }
                    Token::Comma => true,
                    Token::RightParen => false,
                    _ => panic!("unexpected token {}", self.previous()),
                } {}

                match self.advance() {
                    Token::Arrow => (),
                    _ => panic!("unexpected token {}", self.previous()),
                }

                match self.advance() {
                    Token::LCurly => (),
                    _ => panic!("unexpected token {}", self.previous()),
                }

                let mut body: Vec<Expression> = Vec::new();

                while match self.peek() {
                    Token::RCurly => {
                        self.advance();
                        false
                    }
                    _ => {
                        body.push(self.expression());
                        true
                    }
                } {}
                Expression::FuncDecl(expression::FuncDecl { body, params })
            }
            _ => self.func_call(),
        }
    }

    fn func_call(&mut self) -> Expression {
        let mut expr = self.primary();

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
                                args.push(self.expression());
                                true
                            }
                        } {}

                        expr = Expression::FuncCall(expression::FuncCall {
                            calee: Box::new(expr),
                            args,
                        });
                    }
                    _ => panic!("unexpected token {}", self.peek()),
                }

                true
            }
            _ => false,
        } {}
        expr
    }

    fn primary(&mut self) -> Expression {
        match self.advance() {
            Token::Numeric(val) => Expression::Numeric(*val),
            Token::LeftParen => {
                let expr = Expression::Grouping(Box::new(self.expression()));

                match self.advance() {
                    Token::RightParen => (),
                    _ => panic!("Unmatched paren"),
                };

                expr
            }
            Token::Identifier(literal) => Expression::Identifier(literal.to_string()),
            Token::String(literal) => Expression::String(literal.to_string()),
            Token::True => Expression::Bool(true),
            Token::False => Expression::Bool(false),
            Token::Break => Expression::Break,
            _ => panic!("unexpected token {}", self.previous()),
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
