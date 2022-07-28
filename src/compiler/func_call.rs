use crate::{
    expression::{self, Expression, FuncDecl},
    llvm, parser,
    visitor::FuncCallVisitor,
    visitor::Visitor,
};

use super::{scope::Scope, Compiler, CompilerError, CompilerResult, Value};

impl Compiler {}

impl FuncCallVisitor<CompilerResult<Value>> for Compiler {
    fn visit_func_call(&mut self, expr: &expression::FuncCall) -> CompilerResult<Value> {
        let name = match &*expr.calee {
            expression::Expression::Identifier(ref name) => Ok(name.clone()),
            _ => Err(CompilerError::TypeError),
        }?;

        let args = self.compile_args(&expr.args)?;

        if let Some(var) = self.get_var(&name) {
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
                        Ok(Value::String(self.builder.build_call(&val, &args, "")))
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
        assert_eq_ir,
        compiler::{scope::Scope, Compiler, Value, MAIN_FUNCTION},
        expression::{Expression, FuncCall},
        remove_whitespace,
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

        let main_fun = compiler.module.add_function(
            MAIN_FUNCTION,
            compiler
                .context
                .function_type(compiler.context.void_type(), &[], false),
        );
        let block = compiler.context.append_basic_block(&main_fun, "");
        compiler.builder.position_builder_at_end(&block);
        compiler.visit_func_call(&func_call).unwrap();
        compiler.builder.build_ret_void();

        compiler.turn_off_optimization();
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
