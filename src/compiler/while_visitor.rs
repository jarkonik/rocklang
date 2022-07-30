use crate::visitor::WhileVisitor;

use super::{Compiler, CompilerResult, Value};

impl WhileVisitor<CompilerResult<Value>> for Compiler {
    fn visit_while(&mut self, expr: &crate::expression::While) -> CompilerResult<Value> {
        todo!()
    }
}
