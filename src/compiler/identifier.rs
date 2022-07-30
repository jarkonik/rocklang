use crate::visitor::IdentifierVisitor;

use super::{Compiler, CompilerResult, Value};

impl IdentifierVisitor<CompilerResult<Value>> for Compiler {
    fn visit_identifier(&mut self, expr: &str) -> CompilerResult<Value> {
		todo!()
	}
}
