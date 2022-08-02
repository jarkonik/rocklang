use crate::{
    expression::{self, Expression},
    llvm, parser,
    visitor::FuncCallVisitor,
};

use super::{Compiler, CompilerError, CompilerResult, LLVMCompiler, Value};

fn compile_args<T: LLVMCompiler>(
    compiler: &mut T,
    args: &[Expression],
) -> CompilerResult<Vec<llvm::Value>> {
    args.iter()
        .map(|arg| {
            let val = match compiler.walk(arg)? {
                Value::Void => Err(CompilerError::TypeError)?,
                Value::String(n) => n,
                Value::Numeric(n) => n,
                Value::Bool(_) => todo!(),
                Value::Function { .. } => todo!(),
                Value::Vec(n) => n,
                Value::Break => todo!(),
                Value::Ptr(_) => todo!(),
            };

            Ok(val)
        })
        .collect()
}

fn compile_func_call<T: LLVMCompiler>(
    compiler: &mut T,
    expr: &expression::FuncCall,
) -> CompilerResult<Value> {
    let name = match &*expr.calee {
        expression::Expression::Identifier(ref name) => Ok(name.clone()),
        _ => Err(CompilerError::TypeError),
    }?;

    let var = compiler.get_var(&name)?;

    let args = compile_args(compiler, &expr.args)?;

    let builder = compiler.builder();

    if let Value::Function {
        return_type, val, ..
    } = var
    {
        let llvm_value = builder.build_call(&val, &args, "");

        let val = match return_type {
            parser::Type::Numeric => Value::Numeric(llvm_value),
            parser::Type::Vector => {
                let value = Value::Vec(llvm_value);
                compiler.track_reference(value);
                value
            }
            parser::Type::Void => Value::Void,
            parser::Type::Function => todo!(),
            parser::Type::Ptr => Value::Ptr(llvm_value),
            parser::Type::Bool => Value::Bool(llvm_value),
            parser::Type::String => {
                let value = Value::String(llvm_value);
                compiler.track_reference(value);
                value
            }
        };

        Ok(val)
    } else {
        Err(CompilerError::TypeError)
    }
}

impl FuncCallVisitor<CompilerResult<Value>> for Compiler {
    fn visit_func_call(&mut self, expr: &expression::FuncCall) -> CompilerResult<Value> {
        compile_func_call(self, expr)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::compiler::MAIN_FUNCTION;
    use crate::parser::Type;
    use crate::visitor::*;
    use crate::{
        compiler::{CompilerError, CompilerResult, LLVMCompiler, Value},
        expression::{Expression, FuncCall},
        llvm::{Builder, Context, Module},
    };
    use indoc::indoc;
    use mockall::{
        mock,
        predicate::{self, *},
    };
    use pretty_assertions::assert_eq;

    mock_compiler!();

    macro_rules! test_func_call {
        ($return_type: expr, $arg_types: expr, $args: expr) => {{
            let context = Context::new();
            let module = context.create_module("main");
            let builder = context.create_builder();
            let mut compiler = MockCompiler::new();

            compiler.expect_context().return_const(context);
            compiler.expect_builder().return_const(builder);
            compiler.expect_module().return_const(module);

            let fun_type = compiler.context().function_type(
                match $return_type {
                    Type::Void => compiler.context().void_type(),
                    Type::Numeric => compiler.context().double_type(),
                    Type::Bool => compiler.context().i1_type(),
                    Type::Ptr => compiler.context().void_type().pointer_type(0),
                    _ => todo!(),
                },
                &$arg_types,
                false,
            );
            let fun = compiler.module().add_function("", fun_type);

            let fun_value = Value::Function {
                return_type: $return_type,
                typ: fun_type,
                val: fun,
            };

            compiler
                .expect_get_var()
                .with(predicate::eq("test_fun"))
                .times(1)
                .return_const_st(Ok(fun_value));

            let const_double = Value::Numeric(compiler.context().const_double(3.0));
            compiler.expect_walk().return_const_st(Ok(const_double));

            let val: Value;
            in_main_function!(compiler.context(), compiler.module(), compiler.builder(), {
                let func_call = FuncCall {
                    calee: Box::new(Expression::Identifier("test_fun".to_string())),
                    args: $args,
                };
                val = compile_func_call(&mut compiler, &func_call)?;
            });

            (compiler.module().to_string(), val)
        }};
    }

    #[test]
    fn test_func_void_no_args_call() -> Result<(), CompilerError> {
        let (ir, return_value) = test_func_call!(Type::Void, vec![], vec![]);
        assert!(matches!(return_value, Value::Void));
        assert_eq_ir!(
            ir,
            r#"
            declare void @0()

            define void @main() {
              call void @0()
              ret void
            }
            "#
        );
        Ok(())
    }

    #[test]
    fn test_func_numeric_no_args_call() -> Result<(), CompilerError> {
        let (ir, return_value) = test_func_call!(Type::Numeric, vec![], vec![]);
        assert!(matches!(return_value, Value::Numeric(_)));
        assert_eq_ir!(
            ir,
            r#"
            declare double @0()

            define void @main() {
              %1 = call double @0()
              ret void
            }
            "#
        );
        Ok(())
    }

    #[test]
    fn test_func_boolean_no_args_call() -> Result<(), CompilerError> {
        let (ir, return_value) = test_func_call!(Type::Bool, vec![], vec![]);
        assert!(matches!(return_value, Value::Bool(_)));
        assert_eq_ir!(
            ir,
            r#"
            declare i1 @0()

            define void @main() {
              %1 = call i1 @0()
              ret void
            }
            "#
        );
        Ok(())
    }

    #[test]
    fn test_func_ptr_no_args_call() -> Result<(), CompilerError> {
        let (ir, return_value) = test_func_call!(Type::Ptr, vec![], vec![]);
        assert!(matches!(return_value, Value::Ptr(_)));
        assert_eq_ir!(
            ir,
            r#"
            declare void* @0()

            define void @main() {
              %1 = call void* @0()
              ret void
            }
            "#
        );
        Ok(())
    }
}
