use crate::{
    expression::{self, Expression},
    llvm,
    visitor::AssignmentVisitor,
};

use super::{variable::Variable, Compiler, CompilerError, CompilerResult, LLVMCompiler, Value};

fn compile_assignment<T: LLVMCompiler>(
    compiler: &mut T,
    expr: &crate::expression::Assignment,
) -> CompilerResult<Value> {
    let right = compiler.walk(&expr.right)?;

    let ptr = compiler
        .builder()
        .build_alloca(right.llvm_type(compiler.context()), "");
    compiler.builder().create_store(right.into(), &ptr);

    let var = match right {
        Value::String(_) => Variable::String(ptr),
        Value::Numeric(_) => Variable::Numeric(ptr),
        Value::Bool(_) => Variable::Bool(ptr),
        Value::Function {
            return_type, typ, ..
        } => Variable::Function {
            val: llvm::Function(ptr.0),
            typ,
            return_type,
        },
        Value::Vec(_) => Variable::Vec(ptr),
        Value::Ptr(_) => Variable::Ptr(ptr),
        Value::Void | Value::Break => Err(CompilerError::TypeError)?,
    };

    if let Expression::Identifier(name) = &*expr.left {
        compiler.set_var(name, var);
    } else {
        Err(CompilerError::TypeError)?
    }

    if let expression::Expression::FuncDecl(e) = &*expr.right {
        compiler.build_function(right, &*e)?
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
    use crate::compiler::Variable;
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
                predicate::function(|x| matches!(x, Variable::Numeric(_))),
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
