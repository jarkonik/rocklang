use crate::expression::{self, Expression};
use crate::parser::Program;

pub trait BinaryVisitor<T> {
    fn visit_binary(&mut self, expr: &expression::Binary) -> T;
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
    fn visit_conditional(&mut self, expr: &expression::Conditional) -> T;
}

pub trait UnaryVisitor<T> {
    fn visit_unary(&mut self, expr: &expression::Unary) -> T;
}

pub trait GroupingVisitor<T> {
    fn visit_grouping(&mut self, expr: &expression::Expression) -> T;
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
{
    fn visit_while(&mut self, expr: &expression::While) -> T;
    fn visit_identifier(&mut self, expr: &str) -> T;
    fn visit_bool(&mut self, expr: &bool) -> T;
    fn visit_break(&mut self) -> T;
    fn visit_func_decl(&mut self, body: &expression::FuncDecl) -> T;
    fn visit_load(&mut self, name: &str) -> T;
    fn visit_extern(&mut self, name: &expression::Extern) -> T;

    fn walk(&mut self, expr: &Expression) -> T {
        match expr {
            Expression::Binary(expr) => self.visit_binary(expr),
            Expression::Unary(expr) => self.visit_unary(expr),
            Expression::Grouping(expr) => self.visit_grouping(expr),
            Expression::FuncCall(expr) => self.visit_func_call(expr),
            Expression::Numeric(expr) => self.visit_numeric(expr),
            Expression::Assignment(expr) => self.visit_assignment(expr),
            Expression::Identifier(expr) => self.visit_identifier(expr),
            Expression::Conditional(expr) => self.visit_conditional(expr),
            Expression::String(expr) => self.visit_string(expr),
            Expression::Bool(expr) => self.visit_bool(expr),
            Expression::Break => self.visit_break(),
            Expression::While(expr) => self.visit_while(expr),
            Expression::FuncDecl(expr) => self.visit_func_decl(expr),
            Expression::Load(expr) => self.visit_load(expr),
            Expression::Extern(expr) => self.visit_extern(expr),
        }
    }
}
