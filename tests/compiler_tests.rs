use rocklang::compiler::{Compile, Compiler};
use rocklang::expression::{
    Assignment, Binary, Conditional, Expression, FuncCall, FuncDecl, Operator,
};
use rocklang::parser::{Param, Program, Type};

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
fn it_compiles_numeric_asignment() {
    let program = Program {
        body: vec![Expression::Assignment(Assignment {
            left: Box::new(Expression::Identifier("x".to_string())),
            right: Box::new(Expression::Numeric(5.0)),
        })],
    };

    let mut compiler = Compiler::new(program);
    compiler.no_opt();
    compiler.compile().unwrap();

    assert_eq!(
        remove_whitespace(&compiler.ir_string()),
        remove_whitespace(
            "
		; ModuleID = 'main'\nsource_filename = \"main\"
		target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"

		define void @__main__() {
			entry:
			%x = alloca double, align 8
			store double 5.000000e+00, double* %x , align 8
			ret void
		}
	"
        )
    );
}

#[test]
fn it_compiles_numeric_to_numeric_asignment() {
    let program = Program {
        body: vec![
            Expression::Assignment(Assignment {
                left: Box::new(Expression::Identifier("x".to_string())),
                right: Box::new(Expression::Numeric(5.0)),
            }),
            Expression::Assignment(Assignment {
                left: Box::new(Expression::Identifier("y".to_string())),
                right: Box::new(Expression::Identifier("x".to_string())),
            }),
        ],
    };

    let mut compiler = Compiler::new(program);
    compiler.no_opt();
    compiler.compile().unwrap();

    assert_eq!(
        remove_whitespace(&compiler.ir_string()),
        remove_whitespace(
            "
		; ModuleID = 'main'\nsource_filename = \"main\"
		target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"
		define void @__main__() {
			entry:
			%x = alloca double, align 8
			store double 5.000000e+00, double* %x , align 8
			%0 = loaddouble, double* %x, align 8
			%y = alloca double, align 8
			store double %0, double* %y, align 8
			ret void
		}
	"
        )
    );
}

#[test]
fn it_compiles_new_vec_being_passed_as_fun_arg() {
    let program = Program {
        body: vec![
            Expression::Assignment(Assignment {
                left: Box::new(Expression::Identifier("f".to_string())),
                right: Box::new(Expression::FuncDecl(FuncDecl {
                    body: vec![],
                    return_type: Type::Null,
                    params: vec![Param {
                        name: "v".to_string(),
                        typ: Type::Vector,
                    }],
                })),
            }),
            Expression::FuncCall(FuncCall {
                calee: Box::new(Expression::Identifier("f".to_string())),
                args: vec![Expression::FuncCall(FuncCall {
                    calee: Box::new(Expression::Identifier("vecnew".to_string())),
                    args: vec![],
                })],
            }),
        ],
    };

    let mut compiler = Compiler::new(program);
    compiler.compile().unwrap();
}

#[test]
fn it_compiles_new_vec_being_passed_as_variable() {
    let program = Program {
        body: vec![
            Expression::Assignment(Assignment {
                left: Box::new(Expression::Identifier("f".to_string())),
                right: Box::new(Expression::FuncDecl(FuncDecl {
                    body: vec![],
                    return_type: Type::Null,
                    params: vec![Param {
                        name: "v".to_string(),
                        typ: Type::Vector,
                    }],
                })),
            }),
            Expression::Assignment(Assignment {
                left: Box::new(Expression::Identifier("vecinvar".to_string())),
                right: Box::new(Expression::FuncCall(FuncCall {
                    calee: Box::new(Expression::Identifier("vecnew".to_string())),
                    args: vec![],
                })),
            }),
            Expression::FuncCall(FuncCall {
                calee: Box::new(Expression::Identifier("f".to_string())),
                args: vec![Expression::Identifier("vecinvar".to_string())],
            }),
        ],
    };

    let mut compiler = Compiler::new(program);
    compiler.compile().unwrap();
}

#[test]
fn it_compiles_recursive_fun() {
    let program = Program {
        body: vec![Expression::Assignment(Assignment {
            left: Box::new(Expression::Identifier("f".to_string())),
            right: Box::new(Expression::FuncDecl(FuncDecl {
                return_type: Type::Null,
                params: vec![],
                body: vec![Expression::FuncCall(FuncCall {
                    calee: Box::new(Expression::Identifier("f".to_string())),
                    args: vec![],
                })],
            })),
        })],
    };

    let mut compiler = Compiler::new(program);
    compiler.compile().unwrap();
}

#[test]
fn it_compiles_grouping_expressions() {
    let program = Program {
        body: vec![Expression::Assignment(Assignment {
            left: Box::new(Expression::Identifier("x".to_string())),
            right: Box::new(Expression::Grouping(Box::new(Expression::Binary(Binary {
                left: Box::new(Expression::Numeric(10.0)),
                operator: Operator::NotEqual,
                right: Box::new(Expression::Numeric(2.0)),
            })))),
        })],
    };

    let mut compiler = Compiler::new(program);
    compiler.compile().unwrap();
}

#[test]
fn it_compiles_coditional() {
    let program = Program {
        body: vec![Expression::Conditional(Conditional {
            predicate: Box::new(Expression::Binary(Binary {
                left: Box::new(Expression::Numeric(10.0)),
                operator: Operator::NotEqual,
                right: Box::new(Expression::Numeric(2.0)),
            })),
            body: vec![Expression::Numeric(10.0)],
            else_body: vec![Expression::Numeric(20.0)],
        })],
    };

    let mut compiler = Compiler::new(program);
    compiler.compile().unwrap();
}

macro_rules! compile_operator {
    ($left_operator:expr, $operator:expr, $rigth_operator:expr) => {
        let program = Program {
            body: vec![Expression::Assignment(Assignment {
                left: Box::new(Expression::Identifier("b".to_string())),
                right: Box::new(Expression::Binary(Binary {
                    left: $left_operator,
                    operator: $operator,
                    right: $rigth_operator,
                })),
            })],
        };

        let mut compiler = Compiler::new(program);
        compiler.compile().unwrap()
    };
}

#[test]
fn it_compiles_operators() {
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::Plus,
        Box::new(Expression::Numeric(2.0))
    );
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::Minus,
        Box::new(Expression::Numeric(2.0))
    );
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::Asterisk,
        Box::new(Expression::Numeric(2.0))
    );
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::LessOrEqual,
        Box::new(Expression::Numeric(2.0))
    );
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::Less,
        Box::new(Expression::Numeric(2.0))
    );
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::Greater,
        Box::new(Expression::Numeric(2.0))
    );
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::GreaterOrEqual,
        Box::new(Expression::Numeric(2.0))
    );
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::Equal,
        Box::new(Expression::Numeric(2.0))
    );
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::Slash,
        Box::new(Expression::Numeric(2.0))
    );
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::Minus,
        Box::new(Expression::Numeric(2.0))
    );
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::NotEqual,
        Box::new(Expression::Numeric(2.0))
    );
}

#[test]
#[should_panic]
fn it_panics_when_adding_numeric_to_string() {
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::Plus,
        Box::new(Expression::String("test".to_string()))
    );
}

#[test]
#[should_panic]
fn it_panics_when_plus_string_to_numeric() {
    compile_operator!(
        Box::new(Expression::String("test".to_string())),
        Operator::Plus,
        Box::new(Expression::Numeric(10.0))
    );
}

#[test]
#[should_panic]
fn it_panics_when_substract_string_from_numeric() {
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::Minus,
        Box::new(Expression::String("test".to_string()))
    );
}

#[test]
#[should_panic]
fn it_panics_when_substract_numeric_from_string() {
    compile_operator!(
        Box::new(Expression::String("test".to_string())),
        Operator::Minus,
        Box::new(Expression::Numeric(10.0))
    );
}

#[test]
#[should_panic]
fn it_panics_when_multiple_numeric_by_string() {
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::Asterisk,
        Box::new(Expression::String("test".to_string()))
    );
}

#[test]
#[should_panic]
fn it_panics_when_multiple_string_to_numeric() {
    compile_operator!(
        Box::new(Expression::String("test".to_string())),
        Operator::Asterisk,
        Box::new(Expression::Numeric(10.0))
    );
}

#[test]
#[should_panic]
fn it_panics_when_less_or_equal_numeric_and_string() {
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::LessOrEqual,
        Box::new(Expression::String("test".to_string()))
    );
}

#[test]
#[should_panic]
fn it_panics_when_less_or_equal_string_to_numeric() {
    compile_operator!(
        Box::new(Expression::String("test".to_string())),
        Operator::LessOrEqual,
        Box::new(Expression::Numeric(10.0))
    );
}

#[test]
#[should_panic]
fn it_panics_when_less_numeric_and_string() {
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::Less,
        Box::new(Expression::String("test".to_string()))
    );
}

#[test]
#[should_panic]
fn it_panics_when_less_string_to_numeric() {
    compile_operator!(
        Box::new(Expression::String("test".to_string())),
        Operator::Less,
        Box::new(Expression::Numeric(10.0))
    );
}

#[test]
#[should_panic]
fn it_panics_when_greater_numeric_and_string() {
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::Greater,
        Box::new(Expression::String("test".to_string()))
    );
}

#[test]
#[should_panic]
fn it_panics_when_greater_string_to_numeric() {
    compile_operator!(
        Box::new(Expression::String("test".to_string())),
        Operator::Greater,
        Box::new(Expression::Numeric(10.0))
    );
}

#[test]
#[should_panic]
fn it_panics_when_greater_or_equal_numeric_and_string() {
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::GreaterOrEqual,
        Box::new(Expression::String("test".to_string()))
    );
}

#[test]
#[should_panic]
fn it_panics_when_greater_or_equal_string_to_numeric() {
    compile_operator!(
        Box::new(Expression::String("test".to_string())),
        Operator::GreaterOrEqual,
        Box::new(Expression::Numeric(10.0))
    );
}

#[test]
#[should_panic]
fn it_panics_when_equal_numeric_and_string() {
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::Equal,
        Box::new(Expression::String("test".to_string()))
    );
}

#[test]
#[should_panic]
fn it_panics_when_equal_string_to_numeric() {
    compile_operator!(
        Box::new(Expression::String("test".to_string())),
        Operator::Equal,
        Box::new(Expression::Numeric(10.0))
    );
}

#[test]
#[should_panic]
fn it_panics_when_slash_numeric_and_string() {
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::Slash,
        Box::new(Expression::String("test".to_string()))
    );
}

#[test]
#[should_panic]
fn it_panics_when_slash_string_to_numeric() {
    compile_operator!(
        Box::new(Expression::String("test".to_string())),
        Operator::Slash,
        Box::new(Expression::Numeric(10.0))
    );
}

#[test]
#[should_panic]
fn it_panics_when_not_equal_numeric_and_string() {
    compile_operator!(
        Box::new(Expression::Numeric(10.0)),
        Operator::NotEqual,
        Box::new(Expression::String("test".to_string()))
    );
}

#[test]
#[should_panic]
fn it_panics_when_not_equal_string_to_numeric() {
    compile_operator!(
        Box::new(Expression::String("test".to_string())),
        Operator::NotEqual,
        Box::new(Expression::Numeric(10.0))
    );
}
