use crate::visitor::GroupingVisitor;

use super::{Compiler, CompilerResult, Value};

impl GroupingVisitor<CompilerResult<Value>> for Compiler {
    fn visit_grouping(&mut self, expr: &crate::expression::Expression) -> CompilerResult<Value> {
        todo!()
    }
}
