use crate::visitor::ConditionalVisitor;

use super::{Compiler, CompilerResult, Value};

impl ConditionalVisitor<CompilerResult<Value>> for Compiler {
    fn visit_conditional(
        &mut self,
        _expr: &crate::expression::Conditional,
    ) -> CompilerResult<Value> {
        todo!()
    }
}
