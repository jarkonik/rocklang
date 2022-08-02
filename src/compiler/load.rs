use crate::visitor::LoadVisitor;

use super::{Compiler, CompilerError, CompilerResult, Value};

impl LoadVisitor<CompilerResult<Value>> for Compiler {
    fn visit_load(&mut self, name: &str) -> CompilerResult<Value> {
        if self.context.load_libary_permanently(name).is_err() {
            Err(CompilerError::LoadLibaryError(name.to_string()))?
        } else {
            Ok(Value::Void)
        }
    }
}
