use crate::expression::{self, Expression};
use crate::parser::Program;

pub trait BinaryVisitor<T> {
    fn visit_binary(&mut self, expr: &expression::Binary) -> T {
        unimplemented!()
    }
}

pub trait FuncCallVisitor<T> {
    fn visit_func_call(&mut self, expr: &expression::FuncCall) -> T {
        unimplemented!()
    }
}

pub trait NumericVisitor<T> {
    fn visit_numeric(&mut self, expr: &f64) -> T {
        unimplemented!()
    }
}

pub trait StringVisitor<T> {
    fn visit_string(&mut self, expr: &str) -> T {
        unimplemented!()
    }
}

pub trait ProgramVisitor<T> {
    fn visit_program(&mut self, program: Program) -> T {
        unimplemented!()
    }
}

pub trait AssignmentVisitor<T> {
    fn visit_assignment(&mut self, expr: &expression::Assignment) -> T {
        unimplemented!()
    }
}

pub trait ConditionalVisitor<T> {
    fn visit_conditional(&mut self, expr: &expression::Conditional) -> T {
        unimplemented!()
    }
}

pub trait UnaryVisitor<T> {
    fn visit_unary(&mut self, expr: &expression::Unary) -> T {
        unimplemented!()
    }
}

pub trait GroupingVisitor<T> {
    fn visit_grouping(&mut self, expr: &expression::Expression) -> T {
        unimplemented!()
    }
}

pub trait WhileVisitor<T> {
    fn visit_while(&mut self, expr: &expression::While) -> T {
        unimplemented!()
    }
}

pub trait IdentifierVisitor<T> {
    fn visit_identifier(&mut self, expr: &str) -> T {
        unimplemented!()
    }
}

pub trait BoolVisitor<T> {
    fn visit_bool(&mut self, expr: &bool) -> T {
        unimplemented!()
    }
}

pub trait BreakVisitor<T> {
    fn visit_break(&mut self) -> T {
        unimplemented!()
    }
}

pub trait FuncDeclVisitor<T> {
    fn visit_func_decl(&mut self, body: &expression::FuncDecl) -> T {
        unimplemented!()
    }
}

pub trait LoadVisitor<T> {
    fn visit_load(&mut self, name: &str) -> T {
        unimplemented!()
    }
}

pub trait ExternVisitor<T> {
    fn visit_extern(&mut self, name: &expression::Extern) -> T {
        unimplemented!()
    }
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
    fn walk(&mut self, expr: &Expression) -> T {
        unimplemented!()
    }
}
