use crate::{llvm, visitor::FuncDeclVisitor};

use super::{utils::get_llvm_type, Compiler, CompilerResult, Value};

impl FuncDeclVisitor<CompilerResult<Value>> for Compiler {
    fn visit_func_decl(&mut self, expr: &crate::expression::FuncDecl) -> CompilerResult<Value> {
        let types: Vec<llvm::Type> = expr
            .params
            .iter()
            .map(|arg| get_llvm_type(&self.context, &arg.typ))
            .collect();

        let fun_type = self.context.function_type(
            get_llvm_type(&self.context, &expr.return_type),
            &types,
            false,
        );

        let fun = Value::Function {
            return_type: expr.return_type,
            typ: fun_type,
            val: self.module.add_function("", fun_type),
        };

        Ok(fun)
    }
}
