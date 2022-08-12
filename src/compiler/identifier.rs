use crate::{llvm, visitor::IdentifierVisitor};

use super::{variable::Variable, Compiler, CompilerResult, LLVMCompiler, Value};

impl IdentifierVisitor<CompilerResult<Value>> for Compiler {
    fn visit_identifier(&mut self, expr: &str) -> CompilerResult<Value> {
        let var = self.get_var(expr)?;

        let ptr: llvm::Value = var.into();

        let val = self.builder.build_load(&ptr, "");

        let result = match var {
            Variable::String(_) => Value::String(val),
            Variable::Numeric(_) => Value::Numeric(val),
            Variable::Bool(_) => Value::Bool(val),
            Variable::Function {
                typ, return_type, ..
            } => Value::Function {
                val: llvm::Function(val.0),
                typ,
                return_type,
            },
            Variable::Vec(_) => Value::Vec(val),
            Variable::Ptr(_) => Value::Ptr(val),
        };
        Ok(result)
    }
}
