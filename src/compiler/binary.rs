use crate::{
    expression::{self, Operator},
    llvm::{self, Builder, Context},
    visitor::{BinaryVisitor, Visitor},
};

use super::{value::Value, Compiler, CompilerError, CompilerResult};

fn compile_binary(
    builder: &Builder,
    operator: &Operator,
    lhs: llvm::Value,
    rhs: llvm::Value,
) -> CompilerResult<Value> {
    match operator {
        expression::Operator::Plus => Ok(Value::Numeric(builder.build_fadd(lhs, rhs, ""))),
        expression::Operator::Minus => Ok(Value::Numeric(builder.build_fsub(lhs, rhs, ""))),
        expression::Operator::Asterisk => Ok(Value::Numeric(builder.build_fmul(lhs, rhs, ""))),
        expression::Operator::Slash => Ok(Value::Numeric(builder.build_fdiv(lhs, rhs, ""))),
        expression::Operator::Mod => todo!(),
        expression::Operator::Equal => Ok(Value::Bool(builder.build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::Equal,
            "",
        ))),
        expression::Operator::NotEqual => Ok(Value::Bool(builder.build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::NotEqual,
            "",
        ))),
        expression::Operator::Less => Ok(Value::Bool(builder.build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::Less,
            "",
        ))),
        expression::Operator::Greater => Ok(Value::Bool(builder.build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::Greater,
            "",
        ))),
        expression::Operator::LessOrEqual => Ok(Value::Bool(builder.build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::LessOrEqual,
            "",
        ))),
        expression::Operator::GreaterOrEqual => Ok(Value::Bool(builder.build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::GreaterOrEqual,
            "",
        ))),
    }
}

impl BinaryVisitor<CompilerResult<Value>> for Compiler {
    fn visit_binary(&mut self, expr: &expression::Binary) -> CompilerResult<Value> {
        let lhs = if let Value::Numeric(n) = self.walk(&expr.left)? {
            n
        } else {
            Err(CompilerError::TypeError)?
        };

        let rhs = if let Value::Numeric(n) = self.walk(&expr.right)? {
            n
        } else {
            Err(CompilerError::TypeError)?
        };

        compile_binary(&self.builder, &expr.operator, lhs, rhs)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::compiler::*;

    #[test]
    fn test_addition() -> CompilerResult<()> {
        let mut compiler = Compiler::default();
        let lhs = compiler.context.const_double(2.);
        let rhs = compiler.context.const_double(3.);

        in_main_function!(compiler, {
            let ptr = compiler
                .builder
                .build_alloca(compiler.context.double_type(), "");
            let val = compile_binary(&compiler.builder, &Operator::Plus, lhs, rhs).unwrap();

            if let Value::Numeric(val) = val {
                compiler.builder.create_store(val, &ptr);
            } else {
                panic!()
            }
        });

        assert_eq_ir!(
            &compiler.ir_string(),
            r#"
            define void @main() {
              %1 = alloca double, align 8
              store double 5.000000e+00, double* %1, align 8
              ret void
            }
            "#
        );

        Ok(())
    }

    #[test]
    fn test_multiplication() -> CompilerResult<()> {
        let mut compiler = Compiler::default();
        let lhs = compiler.context.const_double(2.);
        let rhs = compiler.context.const_double(3.);

        in_main_function!(compiler, {
            let ptr = compiler
                .builder
                .build_alloca(compiler.context.double_type(), "");
            let val = compile_binary(&compiler.builder, &Operator::Asterisk, lhs, rhs).unwrap();

            if let Value::Numeric(val) = val {
                compiler.builder.create_store(val, &ptr);
            } else {
                panic!()
            }
        });

        assert_eq_ir!(
            &compiler.ir_string(),
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
}
