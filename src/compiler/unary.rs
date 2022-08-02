use crate::{
    expression,
    visitor::{UnaryVisitor, Visitor},
};

use super::{Compiler, CompilerError, CompilerResult, Value};

impl UnaryVisitor<CompilerResult<Value>> for Compiler {
    fn visit_unary(&mut self, expr: &crate::expression::Unary) -> CompilerResult<Value> {
        match expr.operator {
            expression::Operator::Minus => {
                let r = match self.walk(&expr.right)? {
                    Value::Numeric(p) => p,
                    _ => Err(CompilerError::TypeError)?,
                };

                Ok(Value::Numeric(self.builder.build_fneg(r, "")))
            }
            _ => Err(CompilerError::TypeError)?,
        }
    }
}
