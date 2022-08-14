use crate::compiler::LLVMCompiler;
use crate::{
    parser::Program,
    visitor::{ProgramVisitor, Visitor},
};

use super::{Compiler, CompilerResult, Value, MAIN_FUNCTION};

impl ProgramVisitor<CompilerResult<Value>> for Compiler {
    fn visit_program(&mut self, program: Program) -> CompilerResult<Value> {
        self.enter_scope();
        self.init_builtins();

        let main_fun = self.module.add_function(
            MAIN_FUNCTION,
            self.context
                .function_type(self.context.void_type(), &[], false),
        );
        let block = self.context.append_basic_block(&main_fun, "");
        self.builder.position_builder_at_end(&block);

        for stmt in program.body {
            self.release_maybe_orphaned();
            self.walk(&stmt)?;
        }
        self.exit_scope()?;

        self.builder.build_ret_void();

        self.verify_function(main_fun)?;

        if self.optimization {
            self.pass_manager.run(&main_fun);
        };

        Ok(Value::Void)
    }
}
