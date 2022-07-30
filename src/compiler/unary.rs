use crate::visitor::UnaryVisitor;

use super::{Compiler, CompilerResult, Value};

impl UnaryVisitor<CompilerResult<Value>> for Compiler {
    fn visit_unary(&mut self, expr: &crate::expression::Unary) -> CompilerResult<Value> {
        todo!()
    }
}
