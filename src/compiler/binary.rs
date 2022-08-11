use crate::{
    expression::{self},
    visitor::BinaryVisitor,
};

use super::{value::Value, Compiler, CompilerError, CompilerResult, LLVMCompiler};

fn compile_binary<T: LLVMCompiler>(
    compiler: &mut T,
    expr: &expression::Binary,
) -> CompilerResult<Value> {
    let lhs = if let Value::Numeric(n) = compiler.walk(&expr.left)? {
        n
    } else {
        Err(CompilerError::TypeError)?
    };

    let rhs = if let Value::Numeric(n) = compiler.walk(&expr.right)? {
        n
    } else {
        Err(CompilerError::TypeError)?
    };

    match expr.operator {
        expression::Operator::Plus => {
            Ok(Value::Numeric(compiler.builder().build_fadd(lhs, rhs, "")))
        }
        expression::Operator::Minus => {
            Ok(Value::Numeric(compiler.builder().build_fsub(lhs, rhs, "")))
        }
        expression::Operator::Asterisk => {
            Ok(Value::Numeric(compiler.builder().build_fmul(lhs, rhs, "")))
        }
        expression::Operator::Slash => {
            Ok(Value::Numeric(compiler.builder().build_fdiv(lhs, rhs, "")))
        }
        expression::Operator::Mod => {
            Ok(Value::Numeric(compiler.builder().build_frem(lhs, rhs, "")))
        }
        expression::Operator::Equal => Ok(Value::Bool(compiler.builder().build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::Equal,
            "",
        ))),
        expression::Operator::NotEqual => Ok(Value::Bool(compiler.builder().build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::NotEqual,
            "",
        ))),
        expression::Operator::Less => Ok(Value::Bool(compiler.builder().build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::Less,
            "",
        ))),
        expression::Operator::Greater => Ok(Value::Bool(compiler.builder().build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::Greater,
            "",
        ))),
        expression::Operator::LessOrEqual => Ok(Value::Bool(compiler.builder().build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::LessOrEqual,
            "",
        ))),
        expression::Operator::GreaterOrEqual => Ok(Value::Bool(compiler.builder().build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::GreaterOrEqual,
            "",
        ))),
    }
}

impl BinaryVisitor<CompilerResult<Value>> for Compiler {
    fn visit_binary(&mut self, expr: &expression::Binary) -> CompilerResult<Value> {
        compile_binary(self, expr)
    }
}

#[cfg(test)]
mod test {
    use mockall::{mock, predicate::*};

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::compiler::Variable;
    use crate::compiler::MAIN_FUNCTION;
    use crate::llvm::{Builder, Context, Module};
    use crate::parser;
    use crate::visitor::*;

    mock_compiler!();

    fn test_binary_operation(
        operator: expression::Operator,
    ) -> Result<(String, Value), CompilerError> {
        let context = Context::new();
        let module = context.create_module("main");
        let builder = context.create_builder();
        let mut compiler = MockCompiler::new();
        compiler.expect_context().return_const(context);
        compiler.expect_builder().return_const(builder);

        let const_double = Value::Numeric(compiler.context().const_double(3.0));
        compiler.expect_walk().return_const_st(Ok(const_double));

        let val: Value;
        in_main_function!(compiler.context(), module, compiler.builder(), {
            val = compile_binary(
                &mut compiler,
                &expression::Binary {
                    left: Box::new(expression::Expression::Numeric(6.0)),
                    operator,
                    right: Box::new(expression::Expression::Numeric(2.0)),
                },
            )?;

            match val {
                Value::Numeric(val) => {
                    let ptr = compiler
                        .builder()
                        .build_alloca(compiler.context().double_type(), "");
                    compiler.builder().create_store(val, &ptr);
                }
                Value::Bool(val) => {
                    let ptr = compiler
                        .builder()
                        .build_alloca(compiler.context().i1_type(), "");
                    compiler.builder().create_store(val, &ptr);
                }
                _ => assert!(false, "Unexpected value"),
            }
        });

        Ok((module.to_string(), val))
    }

    #[test]
    fn test_addition() -> Result<(), CompilerError> {
        let (ir, val) = test_binary_operation(expression::Operator::Plus)?;
        assert!(matches!(val, Value::Numeric(_)));
        assert_eq_ir!(
            ir,
            r#"
            define void @main() {
              %1 = alloca double, align 8
              store double 6.000000e+00, double* %1, align 8
              ret void
            }
            "#
        );
        Ok(())
    }

    #[test]
    fn test_subtraction() -> Result<(), CompilerError> {
        let (ir, val) = test_binary_operation(expression::Operator::Minus)?;
        assert!(matches!(val, Value::Numeric(_)));
        assert_eq_ir!(
            ir,
            r#"
            define void @main() {
              %1 = alloca double, align 8
              store double 0.000000e+00, double* %1, align 8
              ret void
            }
            "#
        );
        Ok(())
    }

    #[test]
    fn test_multiplication() -> Result<(), CompilerError> {
        let (ir, val) = test_binary_operation(expression::Operator::Asterisk)?;
        assert!(matches!(val, Value::Numeric(_)));
        assert_eq_ir!(
            ir,
            r#"
            define void @main() {
              %1 = alloca double, align 8
              store double 9.000000e+00, double* %1, align 8
              ret void
            }
            "#
        );
        Ok(())
    }

    #[test]
    fn test_division() -> Result<(), CompilerError> {
        let (ir, val) = test_binary_operation(expression::Operator::Slash)?;
        assert!(matches!(val, Value::Numeric(_)));
        assert_eq_ir!(
            ir,
            r#"
            define void @main() {
              %1 = alloca double, align 8
              store double 1.000000e+00, double* %1, align 8
              ret void
            }
            "#
        );
        Ok(())
    }

    #[test]
    fn test_remainder() -> Result<(), CompilerError> {
        let (ir, val) = test_binary_operation(expression::Operator::Mod)?;
        assert!(matches!(val, Value::Numeric(_)));
        assert_eq_ir!(
            ir,
            r#"
            define void @main() {
              %1 = alloca double, align 8
              store double 0.000000e+00, double* %1, align 8
              ret void
            }
            "#
        );
        Ok(())
    }

    #[test]
    fn test_equality() -> Result<(), CompilerError> {
        let (ir, val) = test_binary_operation(expression::Operator::Equal)?;
        assert!(matches!(val, Value::Bool(_)));
        assert_eq_ir!(
            ir,
            r#"
            define void @main() {
              %1 = alloca i1, align 1
              store i1 true, i1* %1, align 1
              ret void
            }
            "#
        );
        Ok(())
    }

    #[test]
    fn test_not_equal() -> Result<(), CompilerError> {
        let (ir, val) = test_binary_operation(expression::Operator::NotEqual)?;
        assert!(matches!(val, Value::Bool(_)));
        assert_eq_ir!(
            ir,
            r#"
            define void @main() {
              %1 = alloca i1, align 1
              store i1 false, i1* %1, align 1
              ret void
            }
            "#
        );
        Ok(())
    }

    #[test]
    fn test_less() -> Result<(), CompilerError> {
        let (ir, val) = test_binary_operation(expression::Operator::Less)?;
        assert!(matches!(val, Value::Bool(_)));
        assert_eq_ir!(
            ir,
            r#"
            define void @main() {
              %1 = alloca i1, align 1
              store i1 false, i1* %1, align 1
              ret void
            }
            "#
        );
        Ok(())
    }

    #[test]
    fn test_less_or_equal() -> Result<(), CompilerError> {
        let (ir, val) = test_binary_operation(expression::Operator::LessOrEqual)?;
        assert!(matches!(val, Value::Bool(_)));
        assert_eq_ir!(
            ir,
            r#"
            define void @main() {
              %1 = alloca i1, align 1
              store i1 true, i1* %1, align 1
              ret void
            }
            "#
        );
        Ok(())
    }

    #[test]
    fn test_greater() -> Result<(), CompilerError> {
        let (ir, val) = test_binary_operation(expression::Operator::Greater)?;
        assert!(matches!(val, Value::Bool(_)));
        assert_eq_ir!(
            ir,
            r#"
            define void @main() {
              %1 = alloca i1, align 1
              store i1 false, i1* %1, align 1
              ret void
            }
            "#
        );
        Ok(())
    }

    #[test]
    fn test_greater_or_equal() -> Result<(), CompilerError> {
        let (ir, val) = test_binary_operation(expression::Operator::GreaterOrEqual)?;
        assert!(matches!(val, Value::Bool(_)));
        assert_eq_ir!(
            ir,
            r#"
            define void @main() {
              %1 = alloca i1, align 1
              store i1 true, i1* %1, align 1
              ret void
            }
            "#
        );
        Ok(())
    }
}
