use crate::parser::Span;
use crate::visitor::WhileVisitor;
use crate::{llvm, parser};

use super::{Compiler, CompilerError, CompilerResult, Value};
use crate::compiler::LLVMCompiler;
use crate::visitor::Visitor;

impl WhileVisitor<CompilerResult<Value>> for Compiler {
    fn visit_while(
        &mut self,
        expr: &crate::expression::While,
        span: Span,
    ) -> CompilerResult<Value> {
        let predicate = match self.walk(&expr.predicate)? {
            Value::Bool(b) => b,
            val => Err(CompilerError::TypeError {
                expected: parser::Type::Bool,
                actual: val.get_type(),
                span,
            })?,
        };

        let fun = self.builder().get_insert_block().get_parent();

        let loop_block = self.context().append_basic_block(&fun, "loop");
        let after_loop_block = self.context().append_basic_block(&fun, "afterloop");
        self.after_loop_blocks.push(after_loop_block);

        self.builder
            .build_cond_br(&predicate, &loop_block, &after_loop_block);

        self.builder.position_builder_at_end(&loop_block);

        self.enter_scope();

        let mut is_break = false;
        for stmt in &expr.body {
            self.release_maybe_orphaned();
            if let Value::Break = self.walk(stmt)? {
                is_break = true;
                break;
            };
        }
        self.exit_scope().unwrap();

        let pred: llvm::Value = self.walk(&expr.predicate)?.into();
        if is_break {
            self.builder.build_br(&after_loop_block);
        } else {
            self.builder
                .build_cond_br(&pred, &loop_block, &after_loop_block);
        }
        self.builder.position_builder_at_end(&after_loop_block);
        self.after_loop_blocks.pop();

        Ok(Value::Void)
    }
}
