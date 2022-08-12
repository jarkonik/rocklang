use crate::{llvm, visitor::IdentifierVisitor};

use super::{variable::Variable, Compiler, CompilerError, CompilerResult, LLVMCompiler, Value};

impl IdentifierVisitor<CompilerResult<Value>> for Compiler {
    fn visit_identifier(&mut self, expr: &str) -> CompilerResult<Value> {
        let var = self.get_var(expr);

        let val = match var {
            Some(var) => {
                let val = self.builder.build_load(&var.into(), "");
                Some(match var {
                    Variable::String(_) => Value::String(val),
                    Variable::Numeric(_) => Value::Numeric(val),
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
                })
            }
            None => self.get_param(expr),
        }
        .ok_or(CompilerError::UndefinedIdentifier(expr.to_string()))?;

        Ok(val)
    }
}
