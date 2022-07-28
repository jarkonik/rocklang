use crate::{
    parser::Program,
    visitor::{ProgramVisitor, Visitor},
};

use super::{Compiler, CompilerResult, Value, MAIN_FUNCTION};

impl ProgramVisitor<CompilerResult<Value>> for Compiler {
    fn visit_program(&mut self, program: Program) -> CompilerResult<Value> {
        let main_fun = self.module.add_function(
            MAIN_FUNCTION,
            self.context
                .function_type(self.context.void_type(), &[], false),
        );
        let block = self.context.append_basic_block(&main_fun, "");
        self.builder.position_builder_at_end(&block);

        for stmt in program.body {
            self.walk(&stmt)?;
        }

        self.builder.build_ret_void();
        Ok(Value::Null)
    }
}
