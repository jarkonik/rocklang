use crate::visitor::FuncDeclVisitor;

use super::{Compiler, CompilerResult, Value};

impl FuncDeclVisitor<CompilerResult<Value>> for Compiler {
    fn visit_func_decl(&mut self, body: &crate::expression::FuncDecl) -> CompilerResult<Value> {
        todo!()
    }
}
