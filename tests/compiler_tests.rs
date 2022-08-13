#[macro_use]
extern crate test_utils;

use std::error::Error;

use rocklang::compiler::{Compile, Compiler};

use rocklang::expression::{
    self, Assignment, Binary, Conditional, Expression, FuncCall, FuncDecl, Node, Operator, Unary,
    While,
};
use rocklang::parser::{Param, Program, Span, Type};

#[test]
fn it_compiles_numeric_asignment() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::Assignment(Assignment {
            left: boxed_node!(Expression::Identifier("x".to_string())),
            right: boxed_node!(Expression::Numeric(5.0)),
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.turn_off_optimization();
    compiler.compile().unwrap();

    assert_eq!(
        remove_whitespace!(&compiler.ir_string()),
        remove_whitespace!(
            "
            ;ModuleID='main'source_filename=\"main\"targetdatalayout=\"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"
            @x= global double 0.000000e+00
            define void @__main__() {
                entry:
                store double 5.000000e+00, double*@x, align 8
                ret void
            }
	"
        )
    );
    Ok(())
}

#[test]
fn it_compiles_numeric_to_numeric_asignment() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![
            node!(Expression::Assignment(Assignment {
                left: boxed_node!(Expression::Identifier("x".to_string())),
                right: boxed_node!(Expression::Numeric(5.0)),
            })),
            node!(Expression::Assignment(Assignment {
                left: boxed_node!(Expression::Identifier("y".to_string())),
                right: boxed_node!(Expression::Identifier("x".to_string())),
            })),
        ],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.turn_off_optimization();
    compiler.compile().unwrap();

    assert_eq!(
        remove_whitespace!(&compiler.ir_string()),
        remove_whitespace!(
            "
            ;ModuleID='main'source_filename=\"main\"targetdatalayout=\"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"
            @x = global double 0.000000e+00
            @y = global double 0.000000e+00
            define void @__main__() {
                entry:
                store double 5.000000e+00, double*@x, align 8
                %0= load double, double*@x, align 8
                store double %0, double* @y, align 8
                ret void
            }
	"
        )
    );

    Ok(())
}

#[test]
fn it_panic_numeric_to_numeric_asignment() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::Assignment(Assignment {
            left: boxed_node!(Expression::String("x".to_string())),
            right: boxed_node!(Expression::Numeric(5.0)),
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();

    Ok(())
}

#[test]
fn it_compiles_new_vec_being_passed_as_fun_arg() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![
            node!(Expression::Assignment(Assignment {
                left: boxed_node!(Expression::Identifier("f".to_string())),
                right: boxed_node!(Expression::FuncDecl(FuncDecl {
                    body: vec![],
                    return_type: Type::Void,
                    params: vec![Param {
                        name: "v".to_string(),
                        typ: Type::Vector,
                    }],
                })),
            })),
            node!(Expression::FuncCall(FuncCall {
                calee: boxed_node!(Expression::Identifier("f".to_string())),
                args: vec![node!(Expression::FuncCall(FuncCall {
                    calee: boxed_node!(Expression::Identifier("vec_new".to_string())),
                    args: vec![],
                }))],
            })),
        ],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_compiles_print_function_with_global_string() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::FuncCall(FuncCall {
            calee: boxed_node!(Expression::Identifier("print".to_string())),
            args: vec![node!(Expression::String("name".to_string()))],
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_compiles_print_function_with_string() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::FuncCall(FuncCall {
            calee: boxed_node!(Expression::Identifier("print".to_string())),
            args: vec![node!(Expression::FuncCall(FuncCall {
                calee: boxed_node!(Expression::Identifier("string".to_string())),
                args: vec![node!(Expression::Numeric(10.0))],
            }))],
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_compiles_len_function_when_pass_new_vec() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::FuncCall(FuncCall {
            calee: boxed_node!(Expression::Identifier("len".to_string())),
            args: vec![node!(Expression::FuncCall(FuncCall {
                calee: boxed_node!(Expression::Identifier("vec_new".to_string())),
                args: vec![],
            }))],
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_compiles_vec_get_function() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![
            node!(Expression::Assignment(Assignment {
                left: boxed_node!(Expression::Identifier("z".to_string())),
                right: boxed_node!(Expression::FuncCall(FuncCall {
                    calee: boxed_node!(Expression::Identifier("vec_new".to_string())),
                    args: vec![],
                })),
            })),
            node!(Expression::FuncCall(FuncCall {
                calee: boxed_node!(Expression::Identifier("vec_get".to_string())),
                args: vec![
                    node!(Expression::Identifier("z".to_string())),
                    node!(Expression::Numeric(0.0)),
                ],
            })),
        ],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_compiles_sqrt_funcion() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::FuncCall(FuncCall {
            calee: boxed_node!(Expression::Identifier("sqrt".to_string())),
            args: vec![node!(Expression::Numeric(4.0))],
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_panic_when_more_then_one_arg_pass_to_print_funcion() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::FuncCall(FuncCall {
            calee: boxed_node!(Expression::Identifier("print".to_string())),
            args: vec![
                node!(Expression::String("name".to_string())),
                node!(Expression::String("foo".to_string())),
            ],
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_panics_when_non_sring_type_pass_to_print_funcions() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::FuncCall(FuncCall {
            calee: boxed_node!(Expression::Identifier("print".to_string())),
            args: vec![node!(Expression::Numeric(10.0))],
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_panic_when_zero_args_pass_to_string_funcion() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::FuncCall(FuncCall {
            calee: boxed_node!(Expression::Identifier("string".to_string())),
            args: vec![],
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_panic_when_bool_arg_pass_to_string_funcion() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::FuncCall(FuncCall {
            calee: boxed_node!(Expression::Identifier("string".to_string())),
            args: vec![node!(Expression::Bool(true))],
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_compiles_new_vec_being_passed_as_variable() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![
            node!(Expression::Assignment(Assignment {
                left: boxed_node!(Expression::Identifier("f".to_string())),
                right: boxed_node!(Expression::FuncDecl(FuncDecl {
                    body: vec![],
                    return_type: Type::Void,
                    params: vec![Param {
                        name: "v".to_string(),
                        typ: Type::Vector,
                    }],
                })),
            })),
            node!(Expression::Assignment(Assignment {
                left: boxed_node!(Expression::Identifier("vecinvar".to_string())),
                right: boxed_node!(Expression::FuncCall(FuncCall {
                    calee: boxed_node!(Expression::Identifier("vec_new".to_string())),
                    args: vec![],
                })),
            })),
            node!(Expression::FuncCall(FuncCall {
                calee: boxed_node!(Expression::Identifier("f".to_string())),
                args: vec![node!(Expression::Identifier("vecinvar".to_string()))],
            })),
        ],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_compiles_recursive_fun() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::Assignment(Assignment {
            left: boxed_node!(Expression::Identifier("f".to_string())),
            right: boxed_node!(Expression::FuncDecl(FuncDecl {
                return_type: Type::Void,
                params: vec![],
                body: vec![node!(Expression::FuncCall(FuncCall {
                    calee: boxed_node!(Expression::Identifier("f".to_string())),
                    args: vec![],
                }))],
            })),
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_compiles_while_statment() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::While(While {
            predicate: boxed_node!(Expression::Bool(false)),
            body: vec![],
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_test_visit_func_decl() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![
            node!(Expression::FuncDecl(FuncDecl {
                params: vec![Param {
                    typ: Type::Numeric,
                    name: "n".to_string(),
                }],
                body: vec![node!(Expression::String("n".to_string()))],
                return_type: Type::Numeric,
            })),
            node!(Expression::FuncDecl(FuncDecl {
                params: vec![Param {
                    typ: Type::Vector,
                    name: "n".to_string(),
                }],
                body: vec![node!(Expression::String("n".to_string()))],
                return_type: Type::Vector,
            })),
        ],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_compiles_grouping_expressions() -> Result<(), Box<dyn Error>> {
    let grouping = expression::Grouping(boxed_node!(Expression::Binary(Binary {
        left: boxed_node!(Expression::Numeric(10.0)),
        operator: Operator::NotEqual,
        right: boxed_node!(Expression::Numeric(2.0)),
    })));

    let program = Program {
        body: vec![node!(Expression::Assignment(Assignment {
            left: boxed_node!(Expression::Identifier("x".to_string())),
            right: boxed_node!(Expression::Grouping(grouping)),
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_compiles_coditional() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::Conditional(Conditional {
            predicate: boxed_node!(Expression::Binary(Binary {
                left: boxed_node!(Expression::Numeric(10.0)),
                operator: Operator::NotEqual,
                right: boxed_node!(Expression::Numeric(2.0)),
            })),
            body: vec![node!(Expression::Numeric(10.0))],
            else_body: vec![node!(Expression::Numeric(20.0))],
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

macro_rules! compile_operator {
    ($left_operator:expr, $operator:expr, $rigth_operator:expr) => {
        let program = Program {
            body: vec![node!(Expression::Assignment(Assignment {
                left: boxed_node!(Expression::Identifier("b".to_string())),
                right: boxed_node!(Expression::Binary(Binary {
                    left: $left_operator,
                    operator: $operator,
                    right: $rigth_operator,
                })),
            }))],
        };

        let mut compiler = Compiler::new(program)?;
        compiler.compile().unwrap();
    };
}

#[test]
fn it_compiles_operators() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::Plus,
        boxed_node!(Expression::Numeric(2.0))
    );
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::Minus,
        boxed_node!(Expression::Numeric(2.0))
    );
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::Asterisk,
        boxed_node!(Expression::Numeric(2.0))
    );
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::LessOrEqual,
        boxed_node!(Expression::Numeric(2.0))
    );
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::Less,
        boxed_node!(Expression::Numeric(2.0))
    );
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::Greater,
        boxed_node!(Expression::Numeric(2.0))
    );
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::GreaterOrEqual,
        boxed_node!(Expression::Numeric(2.0))
    );
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::Equal,
        boxed_node!(Expression::Numeric(2.0))
    );
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::Slash,
        boxed_node!(Expression::Numeric(2.0))
    );
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::Minus,
        boxed_node!(Expression::Numeric(2.0))
    );
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::NotEqual,
        boxed_node!(Expression::Numeric(2.0))
    );
    Ok(())
}

#[test]
fn it_panics_when_adding_numeric_to_string() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::Plus,
        boxed_node!(Expression::String("test".to_string()))
    );
    Ok(())
}

#[test]
fn it_panics_when_plus_string_to_numeric() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::String("test".to_string())),
        Operator::Plus,
        boxed_node!(Expression::Numeric(10.0))
    );
    Ok(())
}

#[test]
fn it_panics_when_substract_string_from_numeric() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::Minus,
        boxed_node!(Expression::String("test".to_string()))
    );
    Ok(())
}

#[test]
fn it_panics_when_substract_numeric_from_string() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::String("test".to_string())),
        Operator::Minus,
        boxed_node!(Expression::Numeric(10.0))
    );
    Ok(())
}

#[test]
fn it_panics_when_multiple_numeric_by_string() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::Asterisk,
        boxed_node!(Expression::String("test".to_string()))
    );
    Ok(())
}

#[test]
fn it_panics_when_multiple_string_to_numeric() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::String("test".to_string())),
        Operator::Asterisk,
        boxed_node!(Expression::Numeric(10.0))
    );
    Ok(())
}

#[test]
fn it_panics_when_less_or_equal_numeric_and_string() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::LessOrEqual,
        boxed_node!(Expression::String("test".to_string()))
    );
    Ok(())
}

#[test]
fn it_panics_when_less_or_equal_string_to_numeric() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::String("test".to_string())),
        Operator::LessOrEqual,
        boxed_node!(Expression::Numeric(10.0))
    );
    Ok(())
}

#[test]
fn it_panics_when_less_numeric_and_string() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::Less,
        boxed_node!(Expression::String("test".to_string()))
    );
    Ok(())
}

#[test]
fn it_panics_when_less_string_to_numeric() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::String("test".to_string())),
        Operator::Less,
        boxed_node!(Expression::Numeric(10.0))
    );
    Ok(())
}

#[test]
fn it_panics_when_greater_numeric_and_string() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::Greater,
        boxed_node!(Expression::String("test".to_string()))
    );
    Ok(())
}

#[test]
fn it_panics_when_greater_string_to_numeric() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::String("test".to_string())),
        Operator::Greater,
        boxed_node!(Expression::Numeric(10.0))
    );
    Ok(())
}

#[test]
fn it_panics_when_greater_or_equal_numeric_and_string() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::GreaterOrEqual,
        boxed_node!(Expression::String("test".to_string()))
    );
    Ok(())
}

#[test]
fn it_panics_when_greater_or_equal_string_to_numeric() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::String("test".to_string())),
        Operator::GreaterOrEqual,
        boxed_node!(Expression::Numeric(10.0))
    );
    Ok(())
}

#[test]
fn it_panics_when_equal_numeric_and_string() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::Equal,
        boxed_node!(Expression::String("test".to_string()))
    );
    Ok(())
}

#[test]
fn it_panics_when_equal_string_to_numeric() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::String("test".to_string())),
        Operator::Equal,
        boxed_node!(Expression::Numeric(10.0))
    );
    Ok(())
}

#[test]
fn it_panics_when_slash_numeric_and_string() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::Slash,
        boxed_node!(Expression::String("test".to_string()))
    );
    Ok(())
}

#[test]
fn it_panics_when_slash_string_to_numeric() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::String("test".to_string())),
        Operator::Slash,
        boxed_node!(Expression::Numeric(10.0))
    );
    Ok(())
}

#[test]
fn it_panics_when_not_equal_numeric_and_string() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::Numeric(10.0)),
        Operator::NotEqual,
        boxed_node!(Expression::String("test".to_string()))
    );
    Ok(())
}

#[test]
fn it_panics_when_not_equal_string_to_numeric() -> Result<(), Box<dyn Error>> {
    compile_operator!(
        boxed_node!(Expression::String("test".to_string())),
        Operator::NotEqual,
        boxed_node!(Expression::Numeric(10.0))
    );
    Ok(())
}

#[test]
fn it_compiles_unary_operator() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::Unary(Unary {
            operator: Operator::Minus,
            right: boxed_node!(Expression::Numeric(2.0)),
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_panics_when_pass_string_to_unary() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::Unary(Unary {
            operator: Operator::Minus,
            right: boxed_node!(Expression::String("foo".to_string())),
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_panics_when_wrong_unary_operator() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::Unary(Unary {
            operator: Operator::Plus,
            right: boxed_node!(Expression::Numeric(2.0)),
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_compile_break_in_while() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::While(While {
            predicate: boxed_node!(Expression::Bool(true)),
            body: vec![node!(Expression::Break)],
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}

#[test]
fn it_compiles_ffi_calls() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![
            node!(Expression::Load(String::from("./tests/rockffitestlib.so"))),
            node!(Expression::Assignment(expression::Assignment {
                left: boxed_node!(Expression::Identifier(String::from("sum"))),
                right: boxed_node!(Expression::Extern(expression::Extern {
                    types: [Type::Numeric, Type::Numeric].to_vec(),
                    return_type: Type::Numeric,
                    name: String::from("rockffitest"),
                })),
            })),
            node!(Expression::Assignment(expression::Assignment {
                left: boxed_node!(Expression::Identifier(String::from("getptr"))),
                right: boxed_node!(Expression::Extern(expression::Extern {
                    types: [].to_vec(),
                    return_type: Type::Ptr,
                    name: String::from("getpr"),
                })),
            })),
            node!(Expression::Assignment(expression::Assignment {
                left: boxed_node!(Expression::Identifier(String::from("passptr"))),
                right: boxed_node!(Expression::Extern(expression::Extern {
                    types: [Type::Ptr].to_vec(),
                    return_type: Type::Void,
                    name: String::from("passptr"),
                })),
            })),
            node!(Expression::Assignment(expression::Assignment {
                left: boxed_node!(Expression::Identifier(String::from("passstr"))),
                right: boxed_node!(Expression::Extern(expression::Extern {
                    types: [Type::String].to_vec(),
                    return_type: Type::Void,
                    name: String::from("passstr"),
                })),
            })),
            node!(Expression::FuncCall(expression::FuncCall {
                calee: boxed_node!(Expression::Identifier(String::from("passptr"))),
                args: [node!(Expression::FuncCall(expression::FuncCall {
                    calee: boxed_node!(Expression::Identifier(String::from("getptr"))),
                    args: [].to_vec(),
                }))]
                .to_vec(),
            })),
            node!(Expression::FuncCall(expression::FuncCall {
                calee: boxed_node!(Expression::Identifier(String::from("sum"))),
                args: [
                    node!(Expression::Numeric(2.0)),
                    node!(Expression::Numeric(3.0))
                ]
                .to_vec(),
            })),
            node!(Expression::FuncCall(expression::FuncCall {
                calee: boxed_node!(Expression::Identifier(String::from("passstr"))),
                args: [node!(Expression::String(String::from("test")))].to_vec(),
            })),
        ],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();

    assert_eq!(
        &compiler.ir_string(),
        "
            ; ModuleID = 'main'
source_filename = \"main\"
target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"

@0 = private unnamed_addr constant [5 x i8] c\"test\\00\", align 1

define void @__main__() {
entry:
  %0 = call void* @getpr()
  call void @passptr(void* %0)
  %1 = call double @rockffitest(double 2.000000e+00, double 3.000000e+00)
  call void @passstr(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @0, i64 0, i64 0))
  ret void
}

declare double @rockffitest(double, double)

declare void* @getpr()

declare void @passptr(void*)

declare void @passstr(i8*)
        "
    );
    Ok(())
}

#[test]
fn it_compile_break_in_while_and_if() -> Result<(), Box<dyn Error>> {
    let program = Program {
        body: vec![node!(Expression::While(While {
            predicate: boxed_node!(Expression::Bool(true)),
            body: vec![node!(Expression::Conditional(Conditional {
                predicate: boxed_node!(Expression::Bool(true)),
                body: vec![node!(Expression::Break)],
                else_body: vec![],
            }))],
        }))],
    };

    let mut compiler = Compiler::new(program)?;
    compiler.compile().unwrap();
    Ok(())
}
