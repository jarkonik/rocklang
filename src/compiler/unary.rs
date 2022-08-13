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
            expression::Operator::Minus => {
                let r = match self.walk(&expr.right)? {
                    Value::Numeric(p) => p,
                    val => Err(CompilerError::TypeError {
                        expected: parser::Type::Numeric,
                        actual: val.get_type(),
                        span,
                    })?,
                };

                Ok(Value::Numeric(self.builder.build_fneg(r, "")))
            }
            operator => Err(CompilerError::WrongOperator {
                expected: expression::Operator::Minus,
                actual: operator.clone(),
                span,
            })?,
        }
    }
}
