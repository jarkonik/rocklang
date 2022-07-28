use crate::{
    expression,
    visitor::{BinaryVisitor, Visitor},
};

use super::{value::Value, Compiler, CompilerError, CompilerResult};

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

        match expr.operator {
            expression::Operator::Plus => Ok(Value::Numeric(self.builder.build_fadd(lhs, rhs, ""))),
            expression::Operator::LessOrEqual => todo!(),
            expression::Operator::Less => todo!(),
            expression::Operator::Minus => todo!(),
            expression::Operator::Asterisk => todo!(),
            expression::Operator::Slash => todo!(),
            expression::Operator::Equal => todo!(),
            expression::Operator::Mod => todo!(),
            expression::Operator::NotEqual => todo!(),
            expression::Operator::Greater => todo!(),
            expression::Operator::GreaterOrEqual => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::compiler::*;

    #[test]
    fn test_sum() -> CompilerResult<()> {
        let mut compiler = Compiler::default();

        compiler.scopes.push(Scope::new());

        let expr = expression::Binary {
            left: Box::new(expression::Expression::Numeric(1.0)),
            operator: expression::Operator::Plus,
            right: Box::new(expression::Expression::Numeric(2.0)),
        };

        in_main_function!(compiler, {
            let ptr = compiler
                .builder
                .build_alloca(compiler.context.double_type(), "");
            let val = compiler.visit_binary(&expr).unwrap();
            assert!(matches!(val, Value::Numeric(_)));

            if let Value::Numeric(val) = val {
                compiler.builder.create_store(val, &ptr);
            } else {
                panic!();
            }
        });

        assert_eq_ir!(
            compiler.ir_string(),
            r#"
            ; ModuleID = 'main'
            source_filename = "main"
            target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

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
