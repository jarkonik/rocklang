use crate::expression::{self, Node};
use crate::parser::{Program, Span};

pub trait BinaryVisitor<T> {
    fn visit_binary(&mut self, expr: &expression::Binary, span: Span) -> T;
}

pub trait FuncCallVisitor<T> {
    fn visit_func_call(&mut self, expr: &expression::FuncCall) -> T;
}

pub trait NumericVisitor<T> {
    fn visit_numeric(&mut self, expr: &f64) -> T;
}

pub trait StringVisitor<T> {
    fn visit_string(&mut self, expr: &str) -> T;
}

pub trait ProgramVisitor<T> {
    fn visit_program(&mut self, program: Program) -> T;
}

pub trait AssignmentVisitor<T> {
    fn visit_assignment(&mut self, expr: &expression::Assignment) -> T;
}

pub trait ConditionalVisitor<T> {
    fn visit_conditional(&mut self, expr: &expression::Conditional, span: Span) -> T;
}

pub trait UnaryVisitor<T> {
    fn visit_unary(&mut self, expr: &expression::Unary) -> T;
}

pub trait GroupingVisitor<T> {
    fn visit_grouping(&mut self, expr: &expression::Grouping) -> T;
}

pub trait WhileVisitor<T> {
    fn visit_while(&mut self, expr: &expression::While) -> T;
}

pub trait IdentifierVisitor<T> {
    fn visit_identifier(&mut self, expr: &str) -> T;
}

pub trait BoolVisitor<T> {
    fn visit_bool(&mut self, expr: &bool) -> T;
}

pub trait BreakVisitor<T> {
    fn visit_break(&mut self) -> T;
}

pub trait FuncDeclVisitor<T> {
    fn visit_func_decl(&mut self, body: &expression::FuncDecl) -> T;
}

pub trait LoadVisitor<T> {
    fn visit_load(&mut self, name: &str) -> T;
}

pub trait ExternVisitor<T> {
    fn visit_extern(&mut self, name: &expression::Extern) -> T;
}

pub trait Visitor<T>:
    BinaryVisitor<T>
    + FuncCallVisitor<T>
    + NumericVisitor<T>
    + StringVisitor<T>
    + ProgramVisitor<T>
    + AssignmentVisitor<T>
    + ConditionalVisitor<T>
    + UnaryVisitor<T>
    + GroupingVisitor<T>
    + WhileVisitor<T>
    + IdentifierVisitor<T>
    + BoolVisitor<T>
    + BreakVisitor<T>
    + FuncDeclVisitor<T>
    + LoadVisitor<T>
    + ExternVisitor<T>
{
    fn walk(&mut self, node: &Node) -> T;
}
