use crate::{llvm, visitor::IdentifierVisitor};

use super::{Compiler, CompilerResult, LLVMCompiler, Value};

impl IdentifierVisitor<CompilerResult<Value>> for Compiler {
    fn visit_identifier(&mut self, expr: &str) -> CompilerResult<Value> {
        let var = self.get_var(expr)?;

        let ptr = match var {
            Value::String(n) => n,
            Value::Numeric(n) => n,
            Value::Bool(n) => n,
            Value::Function { val, .. } => llvm::Value(val.0),
            Value::Vec(n) => n,
            Value::Ptr(n) => n,
            Value::Void => unreachable!(),
            Value::Break => unreachable!(),
        };

        let val = self.builder.build_load(&ptr, "");

        let result = match var {
            Value::String(_) => Value::String(val),
            Value::Numeric(_) => Value::Numeric(val),
            Value::Bool(_) => Value::Bool(val),
            Value::Function {
                typ, return_type, ..
            } => Value::Function {
                val: llvm::Function(val.0),
                typ,
                return_type,
            },
            Value::Vec(_) => Value::Vec(val),
            Value::Ptr(_) => Value::Ptr(val),
            Value::Break => unreachable!(),
            Value::Void => unreachable!(),
        };
        Ok(result)
    }
}
