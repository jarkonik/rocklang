use crate::{
    parser::Program,
    visitor::{ProgramVisitor, Visitor},
};

use super::{scope::Scope, Compiler, CompilerResult, Value, MAIN_FUNCTION};

impl ProgramVisitor<CompilerResult<Value>> for Compiler {
    fn visit_program(&mut self, program: Program) -> CompilerResult<Value> {
        self.scopes.push(Scope::new());
        self.init_builtins();

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

        self.scopes
            .pop()
            .unwrap()
            .release_references(&self.module, &self.context, &self.builder);

        self.builder.build_ret_void();

        self.verify_function(main_fun);

        if self.optimization {
            self.pass_manager.run(&main_fun);
        };

        Ok(Value::Null)
    }
}
