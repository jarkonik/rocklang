use crate::{
    expression::{self, Expression},
    parser::Span,
    visitor::AssignmentVisitor,
};

use super::{variable::Variable, Compiler, CompilerError, CompilerResult, LLVMCompiler, Value};

fn compile_assignment<T: LLVMCompiler>(
    compiler: &mut T,
    expr: &crate::expression::Assignment,
    span: Span,
) -> CompilerResult<Value> {
    let right = compiler.walk(&expr.right)?;

    let ptr = if let Expression::Identifier(name) = &expr.left.expression {
        match compiler.get_var(name) {
            Some(ptr) => ptr.into(),
            None => compiler
                .builder()
                .build_alloca(right.llvm_type(compiler.context()), ""),
        }
    } else {
        Err(CompilerError::NonIdentifierAssignment { span: span.clone() })?
    };

    if let Expression::Identifier(name) = &expr.left.expression {
        match compiler.get_var(name) {
            Some(mut var) => {
                match var {
                    Variable::String(val) => {
                        let release = compiler
                            .module()
                            .get_function("release_string_reference")
                            .unwrap();
                        compiler.builder().build_call(
                            &release,
                            &[compiler.builder().build_load(&val, "")],
                            "",
                        );
                    }
                    Variable::Vec(val) => {
                        let release = compiler
                            .module()
                            .get_function("release_vec_reference")
                            .unwrap();

                        compiler.builder().build_call(
                            &release,
                            &[compiler.builder().build_load(&val, "")],
                            "",
                        );
                    }
                    Variable::Numeric(_)
                    | Variable::Bool(_)
                    | Variable::Function { .. }
                    | Variable::Ptr(_) => {}
                }
                compiler.builder().create_store(right.into(), &ptr);
                var.set_value(ptr);
            }
            None => {
                let var = match right {
                    Value::String(_) => Variable::String(ptr),
                    Value::Numeric(_) => Variable::Numeric(ptr),
                    Value::Bool(_) => Variable::Bool(ptr),
                    Value::Function {
                        return_type,
                        typ,
                        val,
                    } => Variable::Function {
                        val,
                        typ,
                        return_type,
                    },
                    Value::Vec(_) => Variable::Vec(ptr),
                    Value::Ptr(_) => Variable::Ptr(ptr),
                    Value::Void | Value::Break => Err(CompilerError::VoidAssignment)?,
                };

                compiler.builder().create_store(right.into(), &ptr);
                compiler.set_var(name, var);
            }
        }

        match right {
            Value::String(val) => {
                let release = compiler
                    .module()
                    .get_function("inc_string_reference")
                    .unwrap();

                compiler.builder().build_call(&release, &[val], "");
            }
            Value::Numeric(_) => {}
            Value::Bool(_) => {}
            Value::Function { .. } => {}
            Value::Vec(val) => {
                let release = compiler.module().get_function("inc_vec_reference").unwrap();

                compiler.builder().build_call(&release, &[val], "");
            }
            Value::Ptr(_) => {}
            Value::Void | Value::Break => Err(CompilerError::VoidAssignment)?,
        };
    } else {
        Err(CompilerError::NonIdentifierAssignment { span })?
    }

    if let expression::Expression::FuncDecl(e) = &expr.right.expression {
        compiler.build_function(right, e)?
    }

    Ok(Value::Void)
}

impl AssignmentVisitor<CompilerResult<Value>> for Compiler {
    fn visit_assignment(
        &mut self,
        expr: &crate::expression::Assignment,
        span: Span,
    ) -> CompilerResult<Value> {
        compile_assignment(self, expr, span)
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
    use crate::parser::Span;
    use crate::{compiler::Variable, expression::Node};
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
        compiler.expect_get_var().return_const_st(None);

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
                    left: Box::new(Node {
                        expression: expression::Expression::Identifier("test".to_string()),
                        span: Default::default(),
                    }),
                    right: Box::new(Node {
                        expression: expression::Expression::Numeric(2.0),
                        span: Default::default(),
                    }),
                },
                Span::default(),
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
