use crate::visitor::StringVisitor;

use super::{Compiler, CompilerResult, Value};

impl StringVisitor<CompilerResult<Value>> for Compiler {
    fn visit_string(&mut self, expr: &str) -> CompilerResult<Value> {
        let with_newlines = expr.to_string().replace("\\n", "\n");
        Ok(Value::ConstString(
            self.builder
                .build_global_string_ptr(with_newlines.as_str(), ""),
        ))
    }
}
