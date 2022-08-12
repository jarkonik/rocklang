use crate::visitor::StringVisitor;

use super::{Compiler, CompilerResult, Value};
use crate::compiler::LLVMCompiler;

impl StringVisitor<CompilerResult<Value>> for Compiler {
    fn visit_string(&mut self, expr: &str) -> CompilerResult<Value> {
        let with_newlines = expr.to_string().replace("\\n", "\n");

        let string_from_c_string = self.module.get_function("string_from_c_string").unwrap();
        let ptr = self
            .builder
            .build_global_string_ptr(with_newlines.as_str(), "");
        let string = Value::String(self.builder.build_call(&string_from_c_string, &[ptr], ""));
        self.track_maybe_orphaned(string);

        Ok(string)
    }
}
