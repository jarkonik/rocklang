use crate::visitor::IdentifierVisitor;

use super::{variable::Variable, Compiler, CompilerError, CompilerResult, LLVMCompiler, Value};

impl IdentifierVisitor<CompilerResult<Value>> for Compiler {
    fn visit_identifier(&mut self, expr: &str) -> CompilerResult<Value> {
        let var = self.get_var(expr);

        let val = match var {
            Some(var) => {
                let val = self
                    .builder
                    .build_load(&var.llvm_type(&self.context), &var.into(), "");
                Some(match var {
                    Variable::String(_) => Value::String(val),
                    Variable::F64(_) => Value::F64(val),
                    Variable::Bool(_) => Value::Bool(val),
                    Variable::Function {
                        typ,
                        return_type,
                        val,
                    } => Value::Function {
                        val,
                        typ,
                        return_type,
                    },
                    Variable::Vec(_) => Value::Vec(val),
                    Variable::Ptr(_) => Value::Ptr(val),
                    Variable::I32(_) => Value::I32(val),
                })
            }
            None => self.get_param(expr),
        }
        .ok_or_else(|| CompilerError::UndefinedIdentifier(expr.to_string()))?;

        Ok(val)
    }
}
