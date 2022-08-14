use crate::{llvm, visitor::ExternVisitor};

use super::{utils::get_llvm_type, Compiler, CompilerResult, Value};

impl ExternVisitor<CompilerResult<Value>> for Compiler {
    fn visit_extern(&mut self, extern_stmt: &crate::expression::Extern) -> CompilerResult<Value> {
        let types: Vec<llvm::Type> = extern_stmt
            .types
            .iter()
            .map(|typ| get_llvm_type(&self.context, typ))
            .collect();

        let fun_type = self.context.function_type(
            get_llvm_type(&self.context, &extern_stmt.return_type),
            &types,
            false,
        );
        let fun = self
            .module
            .add_function(extern_stmt.name.as_str(), fun_type);

        Ok(Value::Function {
            val: fun,
            typ: fun_type,
            return_type: extern_stmt.return_type,
        })
    }
}
