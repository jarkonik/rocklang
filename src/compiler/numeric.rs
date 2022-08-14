use crate::visitor::{F64Visitor, I32Visitor};

use super::{Compiler, CompilerResult, Value};

impl F64Visitor<CompilerResult<Value>> for Compiler {
    fn visit_f64(&mut self, expr: &f64) -> CompilerResult<Value> {
        Ok(Value::F64(self.context.const_double(*expr)))
    }
}

impl I32Visitor<CompilerResult<Value>> for Compiler {
    fn visit_i32(&mut self, expr: &i32) -> CompilerResult<Value> {
        Ok(Value::F64(self.context.const_i32(*expr)))
    }
}
