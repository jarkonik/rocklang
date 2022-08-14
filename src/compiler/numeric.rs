use crate::visitor::F64Visitor;

use super::{Compiler, CompilerResult, Value};

impl F64Visitor<CompilerResult<Value>> for Compiler {
    fn visit_f64(&mut self, expr: &f64) -> CompilerResult<Value> {
        Ok(Value::F64(self.context.const_double(*expr)))
    }
}
