#[macro_export]
macro_rules! remove_whitespace {
    ($s:expr) => {
        $s.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    };
}

#[macro_export]
macro_rules! assert_eq_ir {
    ($result:expr, $valid:expr) => {
        assert_eq!(
            $result,
            concat!(
                indoc!(
                    r#"
            ; ModuleID = 'main'
            source_filename = "main"
        "#,
                ),
                "\n",
                indoc!($valid)
            )
        )
    };
}

#[macro_export]
macro_rules! in_main_function {
    ($context:expr, $module:expr, $builder:expr, $expression:expr) => {
        let main_fun = $module.add_function(
            MAIN_FUNCTION,
            $context.function_type($context.void_type(), &[], false),
        );
        let block = $context.append_basic_block(&main_fun, "");
        $builder.position_builder_at_end(&block);
        $expression
        $builder.build_ret_void();

        main_fun.verify_function().unwrap_or_else(|_x| {
            println!("IR Dump:");
            println!("{}", $module);
            panic!("Function verification failed")
        });
    };
}

#[macro_export]
macro_rules! mock_compiler {
    () => {
        struct Compiler {
            builder: Builder,
            context: Context,
        }

        mock! {
            Compiler { }

            impl NumericVisitor<CompilerResult<Value>> for Compiler {
                fn visit_numeric(&mut self, expr: &f64) -> CompilerResult<Value>;
            }

            impl BinaryVisitor<CompilerResult<Value>> for Compiler {
                fn visit_binary(&mut self, expr: &expression::Binary) -> CompilerResult<Value>;
            }

            impl IdentifierVisitor<CompilerResult<Value>> for Compiler {
                fn visit_identifier(&mut self, expr: &str) -> CompilerResult<Value>;
            }

            impl FuncCallVisitor<CompilerResult<Value>> for Compiler {
                fn visit_func_call(&mut self, expr: &expression::FuncCall) -> CompilerResult<Value>;
            }

            impl FuncDeclVisitor<CompilerResult<Value>> for Compiler {
                fn visit_func_decl(&mut self, body: &expression::FuncDecl) -> CompilerResult<Value>;
            }

            impl StringVisitor<CompilerResult<Value>> for Compiler {
                fn visit_string(&mut self, expr: &str) -> CompilerResult<Value>;
            }

            impl ProgramVisitor<CompilerResult<Value>> for Compiler {
                fn visit_program(&mut self, program: crate::parser::Program) -> CompilerResult<Value>;
            }

            impl AssignmentVisitor<CompilerResult<Value>> for Compiler {
                fn visit_assignment(&mut self, expr: &expression::Assignment) -> CompilerResult<Value>;
            }

            impl ConditionalVisitor<CompilerResult<Value>> for Compiler {
                fn visit_conditional(&mut self, expr: &expression::Conditional) -> CompilerResult<Value>;
            }

            impl UnaryVisitor<CompilerResult<Value>> for Compiler {
                fn visit_unary(&mut self, expr: &expression::Unary) -> CompilerResult<Value>;
            }

            impl GroupingVisitor<CompilerResult<Value>> for Compiler {
                fn visit_grouping(&mut self, expr: &expression::Expression) -> CompilerResult<Value>;
            }

            impl WhileVisitor<CompilerResult<Value>> for Compiler {
                fn visit_while(&mut self, expr: &expression::While) -> CompilerResult<Value>;
            }

            impl BoolVisitor<CompilerResult<Value>> for Compiler {
                fn visit_bool(&mut self, expr: &bool) -> CompilerResult<Value>;
            }

            impl BreakVisitor<CompilerResult<Value>> for Compiler {
                fn visit_break(&mut self) -> CompilerResult<Value>;
            }

            impl LoadVisitor<CompilerResult<Value>> for Compiler {
                fn visit_load(&mut self, name: &str) -> CompilerResult<Value>;
            }

            impl ExternVisitor<CompilerResult<Value>> for Compiler {
                fn visit_extern(&mut self, name: &expression::Extern) -> CompilerResult<Value>;
            }

            impl Visitor<CompilerResult<Value>> for Compiler {
                fn walk(&mut self, expr: &expression::Expression) -> CompilerResult<Value>;
            }

            impl<'a> LLVMCompiler<'a> for Compiler {
                fn builder(&'a self) -> &Builder;
                fn context(&'a self) -> &Context;
            }
        }
    }
}
