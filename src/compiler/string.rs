use std::ffi::c_void;

use crate::visitor::StringVisitor;

use super::{Compiler, CompilerResult, Value};

impl StringVisitor<CompilerResult<Value>> for Compiler {
    fn visit_string(&mut self, expr: &str) -> CompilerResult<Value> {
        let with_newlines = expr.to_string().replace("\\n", "\n");
        self.context.add_symbol(
            "string_from_c_string",
            stdlib::string_from_c_string as *mut c_void,
        );

        let fun_type = self.context.function_type(
            self.context.void_type().pointer_type(0),
            &[self.context.i8_type().pointer_type(0)],
            false,
        );
        let fun = self.module.add_function("string_from_c_string", fun_type);

        let ptr = self
            .builder
            .build_global_string_ptr(with_newlines.as_str(), "");

        let string = self.builder.build_call(&fun, &[ptr], "");

        Ok(Value::ConstString(string))
    }
}
