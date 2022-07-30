use crate::visitor::LoadVisitor;

use super::{Compiler, CompilerResult, Value};

impl LoadVisitor<CompilerResult<Value>> for Compiler {
    fn visit_load(&mut self, name: &str) -> CompilerResult<Value> {
        todo!()
    }
}
