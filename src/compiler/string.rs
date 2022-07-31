use crate::visitor::StringVisitor;

use super::{Compiler, CompilerError, CompilerResult, Value};

impl StringVisitor<CompilerResult<Value>> for Compiler {
    fn visit_string(&mut self, expr: &str) -> CompilerResult<Value> {
        let with_newlines = expr.to_string().replace("\\n", "\n");
        let scope = self.scopes.last_mut().unwrap();

        let fun = if let Value::Function { val, .. } = scope.get("string_from_c_string").unwrap() {
            val
        } else {
            Err(CompilerError::TypeError)?
        };

        let ptr = self
            .builder
            .build_global_string_ptr(with_newlines.as_str(), "");
        let string = Value::String(self.builder.build_call(&fun, &[ptr], ""));
        scope.track_reference(string);

        Ok(string)
    }
}
