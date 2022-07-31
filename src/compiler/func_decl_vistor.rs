use crate::visitor::FuncDeclVisitor;

use super::{Compiler, CompilerResult, Value};

impl FuncDeclVisitor<CompilerResult<Value>> for Compiler {
    fn visit_func_decl(&mut self, _body: &crate::expression::FuncDecl) -> CompilerResult<Value> {
        todo!()
    }
}
