use crate::{llvm, visitor::IdentifierVisitor};

use super::{variable::Variable, Compiler, CompilerResult, LLVMCompiler, Value};

impl IdentifierVisitor<CompilerResult<Value>> for Compiler {
    fn visit_identifier(&mut self, expr: &str) -> CompilerResult<Value> {
        let var = self.get_var(expr)?;

        let ptr = match var {
            Variable::String(n) => n,
            Variable::Numeric(n) => n,
            Variable::Bool(n) => n,
            Variable::Function { val, .. } => llvm::Value(val.0),
            Variable::Vec(n) => n,
            Variable::Ptr(n) => n,
        };

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
