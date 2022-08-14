use crate::visitor::BreakVisitor;

use super::{Compiler, CompilerResult, Value};

impl BreakVisitor<CompilerResult<Value>> for Compiler {
    fn visit_break(&mut self) -> CompilerResult<Value> {
        Ok(Value::Break)
    }
}
