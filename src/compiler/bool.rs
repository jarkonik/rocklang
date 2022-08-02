use crate::visitor::BoolVisitor;

use super::{Compiler, CompilerResult, Value};

impl BoolVisitor<CompilerResult<Value>> for Compiler {
    fn visit_bool(&mut self, expr: &bool) -> CompilerResult<Value> {
        Ok(Value::Bool(self.context.const_bool(*expr)))
    }
}
