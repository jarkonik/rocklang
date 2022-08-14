use crate::visitor::{GroupingVisitor, Visitor};

use super::{Compiler, CompilerResult, Value};

impl GroupingVisitor<CompilerResult<Value>> for Compiler {
    fn visit_grouping(&mut self, expr: &crate::expression::Grouping) -> CompilerResult<Value> {
        self.walk(&*expr.0)
    }
}
