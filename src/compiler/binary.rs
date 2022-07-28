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
