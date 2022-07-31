use crate::visitor::ExternVisitor;

use super::{Compiler, CompilerResult, Value};

impl ExternVisitor<CompilerResult<Value>> for Compiler {
    fn visit_extern(&mut self, _name: &crate::expression::Extern) -> CompilerResult<Value> {
        todo!()
    }
}
