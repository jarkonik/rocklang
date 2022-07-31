use crate::visitor::GroupingVisitor;

use super::{Compiler, CompilerResult, Value};

impl GroupingVisitor<CompilerResult<Value>> for Compiler {
    fn visit_grouping(&mut self, _expr: &crate::expression::Expression) -> CompilerResult<Value> {
        todo!()
    }
}
