use crate::{
    expression::{self, Expression},
    llvm, parser,
    visitor::FuncCallVisitor,
    visitor::Visitor,
};

use super::{Compiler, CompilerError, CompilerResult, Value};

impl Compiler {
    fn compile_args(&mut self, args: &Vec<Expression>) -> Result<Vec<llvm::Value>, CompilerError> {
        args.iter()
            .map(|arg| {
                let val = match self.walk(arg)? {
                    Value::Null => todo!(),
                    Value::String(n) => n,
                    Value::Numeric(n) => n,
                    Value::Bool(_) => todo!(),
                    Value::Function {
                        val,
                        typ,
                        return_type,
                    } => todo!(),
                    Value::Vec(_) => todo!(),
                    Value::Break => todo!(),
                    Value::Ptr(_) => todo!(),
                };

                Ok(val)
            })
            .collect()
    }
}

impl FuncCallVisitor<CompilerResult<Value>> for Compiler {
    fn visit_func_call(&mut self, expr: &expression::FuncCall) -> CompilerResult<Value> {
        let name = match &*expr.calee {
            expression::Expression::Identifier(ref name) => Ok(name.clone()),
            _ => Err(CompilerError::TypeError),
        }?;

        let args = self.compile_args(&expr.args)?;

        let scope = self.scopes.last_mut().unwrap();

        if let Some(var) = scope.get(&name) {
            if let Value::Function {
                return_type, val, ..
            } = var
            {
                match return_type {
                    parser::Type::Numeric => todo!(),
                    parser::Type::Vector => todo!(),
                    parser::Type::Null => {
                        self.builder.build_call(&val, &args, "");
                        Ok(Value::Null)
                    }
                    parser::Type::Function => todo!(),
                    parser::Type::Ptr => todo!(),
                    parser::Type::String => {
                        let value = Value::String(self.builder.build_call(&val, &args, ""));
                        scope.track_reference(value);
                        Ok(value)
                    }
                }
            } else {
                Err(CompilerError::TypeError)
            }
        } else {
            Err(CompilerError::UndefinedIdentifier(name))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        compiler::{scope::Scope, Compiler, Value, MAIN_FUNCTION},
        expression::{Expression, FuncCall},
        visitor::FuncCallVisitor,
    };

    #[test]
    fn test_func_void_no_args_call() {
        let func_call = FuncCall {
            calee: Box::new(Expression::Identifier("test_fun".to_string())),
            args: vec![],
        };

        let mut compiler = Compiler::default();

        compiler.scopes.push(Scope::new());

        let fun_type = compiler
            .context
            .function_type(compiler.context.void_type(), &vec![], false);
        let fun = compiler.module.add_function("", fun_type);

        compiler.set_var(
            "test_fun",
            Value::Function {
                return_type: crate::parser::Type::Null,
                typ: fun_type,
                val: fun,
            },
        );

        in_main_function!(compiler, {
            let val = compiler.visit_func_call(&func_call).unwrap();
            assert!(matches!(val, Value::Null));
        });

        assert_eq_ir!(
            compiler.ir_string(),
            r#"
            ; ModuleID = 'main'
            source_filename = "main"
            target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

            declare void @0()

            define void @main() {
                call void @0()
                ret void
            }
        "#
        );
    }
}
