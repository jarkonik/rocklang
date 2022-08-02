use crate::visitor::{GroupingVisitor, Visitor};

use super::{Compiler, CompilerResult, Value};

impl GroupingVisitor<CompilerResult<Value>> for Compiler {
    fn visit_grouping(&mut self, expr: &crate::expression::Expression) -> CompilerResult<Value> {
        self.walk(expr)
    }
}
