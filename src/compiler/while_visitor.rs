use crate::visitor::WhileVisitor;

use super::{Compiler, CompilerError, CompilerResult, Value};
use crate::compiler::LLVMCompiler;
use crate::visitor::Visitor;

impl WhileVisitor<CompilerResult<Value>> for Compiler {
    fn visit_while(&mut self, expr: &crate::expression::While) -> CompilerResult<Value> {
        let predicate = match self.walk(&expr.predicate)? {
            Value::Bool(b) => b,
            _ => Err(CompilerError::TypeError {
                expected: todo!(),
                actual: todo!(),
                span: todo!(),
            })?,
        };

        let fun = self.builder().get_insert_block().get_parent();

        let loop_block = self.context().append_basic_block(&fun, "loop");
        let after_loop_block = self.context().append_basic_block(&fun, "afterloop");

        self.builder
            .build_cond_br(&predicate, &loop_block, &after_loop_block);

        self.builder.position_builder_at_end(&loop_block);

        self.enter_scope();
        for stmt in &expr.body {
            self.walk(stmt)?;
        }
        self.exit_scope().unwrap();

        self.builder
            .build_cond_br(&predicate, &loop_block, &after_loop_block);
        self.builder.position_builder_at_end(&after_loop_block);

        Ok(Value::Void)
    }
}
