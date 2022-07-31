use crate::{
    expression::{self, Operator},
    llvm::{self, Builder, Context},
    visitor::{BinaryVisitor, Visitor},
};

use super::{value::Value, Compiler, CompilerError, CompilerResult};

trait LLVMCompiler<'a>: Visitor<CompilerResult<Value>> {
    fn builder(&'a self) -> &'a Builder;
    fn context(&'a self) -> &'a Context;
}

fn compile_binary<'a>(
    compiler: &'a mut dyn LLVMCompiler<'a>,
    expr: &expression::Binary,
) -> CompilerResult<Value> {
    let lhs = if let Value::Numeric(n) = compiler.walk(&expr.left)? {
        n
    } else {
        Err(CompilerError::TypeError)?
    };

    let rhs = if let Value::Numeric(n) = compiler.walk(&expr.right)? {
        n
    } else {
        Err(CompilerError::TypeError)?
    };

    match expr.operator {
        expression::Operator::Plus => {
            Ok(Value::Numeric(compiler.builder().build_fadd(lhs, rhs, "")))
        }
        expression::Operator::Minus => {
            Ok(Value::Numeric(compiler.builder().build_fsub(lhs, rhs, "")))
        }
        expression::Operator::Asterisk => {
            Ok(Value::Numeric(compiler.builder().build_fmul(lhs, rhs, "")))
        }
        expression::Operator::Slash => {
            Ok(Value::Numeric(compiler.builder().build_fdiv(lhs, rhs, "")))
        }
        expression::Operator::Mod => todo!(),
        expression::Operator::Equal => Ok(Value::Bool(compiler.builder().build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::Equal,
            "",
        ))),
        expression::Operator::NotEqual => Ok(Value::Bool(compiler.builder().build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::NotEqual,
            "",
        ))),
        expression::Operator::Less => Ok(Value::Bool(compiler.builder().build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::Less,
            "",
        ))),
        expression::Operator::Greater => Ok(Value::Bool(compiler.builder().build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::Greater,
            "",
        ))),
        expression::Operator::LessOrEqual => Ok(Value::Bool(compiler.builder().build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::LessOrEqual,
            "",
        ))),
        expression::Operator::GreaterOrEqual => Ok(Value::Bool(compiler.builder().build_fcmp(
            lhs,
            rhs,
            crate::llvm::Cmp::GreaterOrEqual,
            "",
        ))),
    }
}

impl<'a> LLVMCompiler<'a> for Compiler {
    fn builder(&'a self) -> &'a Builder {
        &self.builder
    }

    fn context(&'a self) -> &'a Context {
        &self.context
    }
}

impl BinaryVisitor<CompilerResult<Value>> for Compiler {
    fn visit_binary(&mut self, expr: &expression::Binary) -> CompilerResult<Value> {
        compile_binary(self, expr)
    }
}

#[cfg(test)]
mod test {
    use mockall::{mock, predicate::*};

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::compiler::MAIN_FUNCTION;
    use crate::expression::Expression;
    use crate::llvm::Module;
    use crate::visitor::*;

    struct MockCompiler {
        builder: Builder,
        context: Context,
    }

    mock! {
        MockCompiler { }

        impl NumericVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_numeric(&mut self, expr: &f64) -> CompilerResult<Value>;
        }

        impl BinaryVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_binary(&mut self, expr: &expression::Binary) -> CompilerResult<Value>;
        }

        impl IdentifierVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_identifier(&mut self, expr: &str) -> CompilerResult<Value>;
        }

        impl FuncCallVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_func_call(&mut self, expr: &expression::FuncCall) -> CompilerResult<Value>;
        }

        impl FuncDeclVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_func_decl(&mut self, body: &expression::FuncDecl) -> CompilerResult<Value>;
        }

        impl StringVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_string(&mut self, expr: &str) -> CompilerResult<Value> ;
        }

        impl ProgramVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_program(&mut self, program: crate::parser::Program) -> CompilerResult<Value> ;
        }

        impl AssignmentVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_assignment(&mut self, expr: &expression::Assignment) -> CompilerResult<Value> ;
        }

        impl ConditionalVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_conditional(&mut self, expr: &expression::Conditional) -> CompilerResult<Value> ;
        }

        impl UnaryVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_unary(&mut self, expr: &expression::Unary) -> CompilerResult<Value> ;
        }

        impl GroupingVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_grouping(&mut self, expr: &expression::Expression) -> CompilerResult<Value> ;
        }

        impl WhileVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_while(&mut self, expr: &expression::While) -> CompilerResult<Value> ;
        }

        impl BoolVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_bool(&mut self, expr: &bool) -> CompilerResult<Value> ;
        }

        impl BreakVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_break(&mut self) -> CompilerResult<Value> ;
        }

        impl LoadVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_load(&mut self, name: &str) -> CompilerResult<Value>;
        }

        impl ExternVisitor<CompilerResult<Value>> for MockCompiler {
            fn visit_extern(&mut self, name: &expression::Extern) -> CompilerResult<Value> ;
        }

        impl Visitor<CompilerResult<Value>> for MockCompiler {
            fn walk(&mut self, expr: &expression::Expression) -> CompilerResult<Value> ;
        }

        impl<'a> LLVMCompiler<'a> for MockCompiler {
            fn builder(&'a self) -> &Builder;
            fn context(&'a self) -> &Context;
        }
    }

    #[test]
    fn test_addition() -> CompilerResult<()> {
        let context = Context::new();
        let module = context.create_module("main");
        let builder = context.create_builder();
        let mut compiler = MockMockCompiler::new();

        compiler
            .expect_walk()
            .return_const_st(Ok(Value::Numeric(context.const_float(3.0))));
        compiler.expect_context().return_const(context);
        compiler.expect_builder().return_const(builder);

        let lhs = compiler.context().const_double(2.);
        let rhs = compiler.context().const_double(3.);

        in_main_function!(compiler.context(), module, compiler.builder(), {
            let ptr = compiler
                .builder()
                .build_alloca(compiler.context().double_type(), "");
            let val = compile_binary(
                &mut compiler,
                &expression::Binary {
                    left: Box::new(expression::Expression::Numeric(6.0)),
                    operator: expression::Operator::Plus,
                    right: Box::new(expression::Expression::Numeric(2.0)),
                },
            )
            .unwrap();

            if let Value::Numeric(val) = val {
                compiler.builder().create_store(val, &ptr);
            } else {
                panic!()
            }
        });

        assert_eq_ir!(
            format!("{}", module),
            r#"
            define void @main() {
              %1 = alloca double, align 8
              store double 8.000000e+00, double* %1, align 8
              ret void
            }
            "#
        );

        Ok(())
    }

    #[test]
    fn test_multiplication() -> CompilerResult<()> {
        let context = Context::new();
        let module = context.create_module("main");
        let builder = context.create_builder();
        let mut compiler = MockMockCompiler::new();

        compiler
            .expect_walk()
            .return_const_st(Ok(Value::Numeric(context.const_float(3.0))));
        compiler.expect_context().return_const(context);
        compiler.expect_builder().return_const(builder);

        let lhs = compiler.context().const_double(2.);
        let rhs = compiler.context().const_double(3.);

        in_main_function!(compiler.context(), module, compiler.builder(), {
            let ptr = compiler
                .builder()
                .build_alloca(compiler.context().double_type(), "");
            let val = compile_binary(
                &mut compiler,
                &expression::Binary {
                    left: Box::new(expression::Expression::Numeric(6.0)),
                    operator: expression::Operator::Asterisk,
                    right: Box::new(expression::Expression::Numeric(2.0)),
                },
            )
            .unwrap();

            if let Value::Numeric(val) = val {
                compiler.builder().create_store(val, &ptr);
            } else {
                panic!()
            }
        });

        assert_eq_ir!(
            format!("{}", module),
            r#"
            define void @main() {
              %1 = alloca double, align 8
              store double 1.600000e+01, double* %1, align 8
              ret void
            }
            "#
        );

        Ok(())
    }
}
