use crate::{expression::Expression, visitor::AssignmentVisitor};

use super::{Compiler, CompilerError, CompilerResult, LLVMCompiler, Value};

fn compile_assignment<T: LLVMCompiler>(
    compiler: &mut T,
    expr: &crate::expression::Assignment,
) -> CompilerResult<Value> {
    let val = match compiler.walk(&expr.right)? {
        Value::Numeric(val) => {
            let ptr = compiler
                .builder()
                .build_alloca(compiler.context().double_type(), "");
            Value::Numeric(compiler.builder().create_store(val, &ptr))
        }
        Value::Bool(val) => {
            let ptr = compiler
                .builder()
                .build_alloca(compiler.context().i1_type(), "");
            Value::Bool(compiler.builder().create_store(val, &ptr))
        }
        Value::Void => Err(CompilerError::TypeError)?,
        Value::String(_) => todo!(),
        Value::Function { .. } => todo!(),
        Value::Vec(val) => {
            let ptr = compiler
                .builder()
                .build_alloca(compiler.context().void_type().pointer_type(0), "");
            Value::Vec(compiler.builder().create_store(val, &ptr))
        }
        Value::Break => todo!(),
        Value::Ptr(_) => todo!(),
    };

    if let Expression::Identifier(name) = &*expr.left {
        compiler.set_var(name, val);
    } else {
        Err(CompilerError::TypeError)?
    }

    Ok(Value::Void)
}

impl AssignmentVisitor<CompilerResult<Value>> for Compiler {
    fn visit_assignment(&mut self, expr: &crate::expression::Assignment) -> CompilerResult<Value> {
        compile_assignment(self, expr)
    }
}

#[cfg(test)]
mod test {
    use mockall::{
        mock,
        predicate::{self, *},
    };

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::compiler::MAIN_FUNCTION;
    use crate::llvm::{Builder, Context, Module};
    use crate::parser;
    use crate::{expression, visitor::*};

    mock_compiler!();

    #[test]
    fn test_numeric_assignment() -> Result<(), CompilerError> {
        let context = Context::new();
        let module = context.create_module("main");
        let builder = context.create_builder();
        let mut compiler = MockCompiler::new();
        compiler.expect_context().return_const(context);
        compiler.expect_builder().return_const(builder);

        let const_double = Value::Numeric(compiler.context().const_double(3.0));
        compiler.expect_walk().return_const_st(Ok(const_double));
        compiler
            .expect_set_var()
            .with(
                predicate::eq("test"),
                predicate::function(|x| matches!(x, Value::Numeric(_))),
            )
            .return_const(());

        let val: Value;
        in_main_function!(compiler.context(), module, compiler.builder(), {
            val = compile_assignment(
                &mut compiler,
                &expression::Assignment {
                    left: Box::new(expression::Expression::Identifier("test".to_string())),
                    right: Box::new(expression::Expression::Numeric(2.0)),
                },
            )?;

            assert!(matches!(val, Value::Void));
        });

        assert_eq_ir!(
            module.to_string(),
            r#"
            define void @main() {
              %1 = alloca double, align 8
              store double 3.000000e+00, double* %1, align 8
              ret void
            }
            "#
        );

        Ok(())
    }
}
