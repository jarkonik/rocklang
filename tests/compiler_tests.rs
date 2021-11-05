use rocklang::compiler::{Compile, Compiler};
use rocklang::expression::{Assignment, Expression, FuncCall, FuncDecl};
use rocklang::parser::{Param, Program, Type};

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
