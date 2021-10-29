use crate::parser::Param;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum Operator {
    Plus,
    LessOrEqual,
    Less,
    Minus,
    Asterisk,
    Slash,
    Equal,
    Mod,
    NotEqual,
    Greater,
}

#[derive(Serialize, Debug, Clone)]
pub struct Binary {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Conditional {
    pub predicate: Box<Expression>,
    pub body: Vec<Expression>,
    pub else_body: Vec<Expression>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Assignment {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Serialize, Debug, Clone)]
pub struct While {
    pub predicate: Box<Expression>,
    pub body: Vec<Expression>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Unary {
    pub operator: Operator,
    pub right: Box<Expression>,
}

#[derive(Serialize, Debug, Clone)]
pub struct FuncCall {
    pub calee: Box<Expression>,
    pub args: Vec<Expression>,
}

#[derive(Serialize, Debug, Clone)]
pub struct FuncDecl {
    pub params: Vec<Param>,
    pub body: Vec<Expression>,
}

#[derive(Serialize, Debug, Clone)]
pub enum Expression {
    Break,
    Bool(bool),
    String(String),
    Identifier(String),
    Numeric(f64),
    Conditional(Conditional),
    Assignment(Assignment),
    Binary(Binary),
    While(While),
    Unary(Unary),
    Grouping(Box<Expression>),
    FuncCall(FuncCall),
    FuncDecl(FuncDecl),
}
