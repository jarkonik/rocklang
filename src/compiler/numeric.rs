use crate::visitor::NumericVisitor;

use super::{Compiler, CompilerResult, Value};

impl NumericVisitor<CompilerResult<Value>> for Compiler {
    fn visit_numeric(&mut self, expr: &f64) -> CompilerResult<Value> {
        Ok(Value::Numeric(self.context.const_double(*expr)))
    }
}
