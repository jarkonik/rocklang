use crate::{
    expression,
    parser::{self, Span},
    visitor::{UnaryVisitor, Visitor},
};

use super::{Compiler, CompilerError, CompilerResult, Value};

impl UnaryVisitor<CompilerResult<Value>> for Compiler {
    fn visit_unary(
        &mut self,
        expr: &crate::expression::Unary,
        span: Span,
    ) -> CompilerResult<Value> {
        match &expr.operator {
            expression::Operator::Minus => match self.walk(&expr.right)? {
                Value::F64(p) => Ok(Value::F64(self.builder.build_fneg(p, ""))),
                Value::I32(p) => Ok(Value::F64(self.builder.build_neg(p, ""))),
                val => Err(CompilerError::TypeError {
                    expected: parser::Type::F64,
                    actual: val.get_type(),
                    span,
                })?,
            },
            operator => Err(CompilerError::WrongOperator {
                expected: expression::Operator::Minus,
                actual: operator.clone(),
                span,
            })?,
        }
    }
}
