use crate::{expression, visitor::ConditionalVisitor};

use super::{Compiler, CompilerError, CompilerResult, LLVMCompiler, Value};

fn compile_conditional<T: LLVMCompiler>(
    compiler: &mut T,
    expr: &expression::Conditional,
) -> CompilerResult<Value> {
    let predicate = match compiler.walk(&expr.predicate)? {
        Value::Bool(b) => b,
        _ => Err(CompilerError::TypeError)?,
    };

    let fun = compiler.builder().get_insert_block().get_parent();

    let then_block = compiler.context().append_basic_block(&fun, "then");
    let else_block = compiler.context().append_basic_block(&fun, "else");
    let after_if_block = compiler.context().append_basic_block(&fun, "afterif");

    compiler
        .builder()
        .build_cond_br(&predicate, &then_block, &else_block);

    compiler.builder().position_builder_at_end(&then_block);
    compiler.enter_scope();
    for stmt in &expr.body {
        compiler.walk(stmt)?;
    }
    compiler.exit_scope().unwrap();
    compiler.builder().create_br(&after_if_block);

    compiler.builder().position_builder_at_end(&else_block);
    compiler.enter_scope();
    for stmt in &expr.else_body {
        compiler.walk(stmt)?;
    }
    compiler.exit_scope().unwrap();
    compiler.builder().create_br(&after_if_block);
    compiler.builder().position_builder_at_end(&after_if_block);

    Ok(Value::Void)
}

impl ConditionalVisitor<CompilerResult<Value>> for Compiler {
    fn visit_conditional(
        &mut self,
        expr: &crate::expression::Conditional,
    ) -> CompilerResult<Value> {
        compile_conditional(self, expr)
    }
}
