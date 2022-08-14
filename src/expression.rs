use crate::parser::Param;
use crate::parser::Span;
use crate::parser::Type;
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
    GreaterOrEqual,
}

#[derive(Serialize, Debug, Clone)]
pub struct Binary {
    pub left: Box<Node>,
    pub operator: Operator,
    pub right: Box<Node>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Conditional {
    pub predicate: Box<Node>,
    pub body: Vec<Node>,
    pub else_body: Vec<Node>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Assignment {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

#[derive(Serialize, Debug, Clone)]
pub struct While {
    pub predicate: Box<Node>,
    pub body: Vec<Node>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Unary {
    pub operator: Operator,
    pub right: Box<Node>,
}

#[derive(Serialize, Debug, Clone)]
pub struct FuncCall {
    pub calee: Box<Node>,
    pub args: Vec<Node>,
}

#[derive(Serialize, Debug, Clone)]
pub struct FuncDecl {
    pub params: Vec<Param>,
    pub body: Vec<Node>,
    pub return_type: Type,
}

#[derive(Serialize, Debug, Clone)]
pub struct Extern {
    pub types: Vec<Type>,
    pub return_type: Type,
    pub name: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct Grouping(pub Box<Node>);

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
    Grouping(Grouping),
    FuncCall(FuncCall),
    FuncDecl(FuncDecl),
    Load(String),
    Extern(Extern),
}

#[derive(Debug, Serialize, Clone)]
pub struct Node {
    pub expression: Expression,
    pub span: Span,
}
