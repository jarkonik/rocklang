use crate::expression;
use crate::expression::Expression;
use crate::parser::Program;

pub trait Visitor<T> {
    fn visit_binary(&mut self, n: &expression::Binary) -> T;
    fn visit_numeric(&mut self, n: &f64) -> T;
    fn visit_conditional(&mut self, n: &expression::Conditional) -> T;
    fn visit_assignment(&mut self, n: &expression::Assignment) -> T;
    fn visit_unary(&mut self, n: &expression::Unary) -> T;
    fn visit_grouping(&mut self, n: &expression::Expression) -> T;
    fn visit_func_call(&mut self, n: &expression::FuncCall) -> T;
    fn visit_while(&mut self, n: &expression::While) -> T;
    fn visit_identifier(&mut self, n: &str) -> T;
    fn visit_string(&mut self, n: &str) -> T;
    fn visit_bool(&mut self, n: &bool) -> T;
    fn visit_break(&mut self) -> T;
    fn visit_program(&mut self, program: Program) -> T;
    fn visit_func_decl(&mut self, body: &expression::FuncDecl) -> T;
    fn visit_load(&mut self, name: &str) -> T;
    fn visit_extern(&mut self, name: &expression::Extern) -> T;
    fn visit_struct(&mut self, name: &expression::Struct) -> T;

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
            Expression::Struct(expr) => self.visit_struct(expr),
        }
    }
}
