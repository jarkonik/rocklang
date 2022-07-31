use crate::visitor::BoolVisitor;

use super::{Compiler, CompilerResult, Value};

impl BoolVisitor<CompilerResult<Value>> for Compiler {
    fn visit_bool(&mut self, _expr: &bool) -> CompilerResult<Value> {
        todo!()
    }
}
