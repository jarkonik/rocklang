use crate::expression;
use crate::llvm;
use crate::parser;
use crate::parser::Program;
use crate::visitor::Visitor;
use std::error::Error;

const MAIN_FUNCTION: &str = "__main__";

pub trait Compile {
	fn compile(&mut self) -> Result<(), Box<dyn Error>>;
}

pub struct Compiler {
	program: Program,
	engine: llvm::Engine,
	context: llvm::Context,
	module: llvm::Module,
	builder: llvm::Builder,
}

enum Value {
	Null,
	String(llvm::Value),
	Numeric(llvm::Value),
}

impl Visitor<Value> for Compiler {
	fn visit_binary(&mut self, _: &expression::Binary) -> Value {
		todo!()
	}

	fn visit_numeric(&mut self, f: &f64) -> Value {
		Value::Numeric(self.context.const_double(*f))
	}

	fn visit_conditional(&mut self, _: &expression::Conditional) -> Value {
		todo!()
	}

	fn visit_assignment(&mut self, _: &expression::Assignment) -> Value {
		todo!()
	}

	fn visit_unary(&mut self, _: &expression::Unary) -> Value {
		todo!()
	}

	fn visit_grouping(&mut self, _: &expression::Expression) -> Value {
		todo!()
	}

	fn visit_func_call(&mut self, expr: &expression::FuncCall) -> Value {
		match &*expr.calee {
			expression::Expression::Identifier(literal) => match literal.as_str() {
				"print" => {
					if expr.args.len() != 1 {
						panic!("arity 1 expected");
					}
					let res = self.walk(&expr.args[0]);

					match res {
						Value::String(s) => {
							let void_type = self.context.void_type();
							let i8_pointer_type = self.context.i8_type().pointer_type(0);
							let func_type =
								llvm::FunctionType::new(void_type, &[i8_pointer_type], false);
							let printf_func = self
								.module
								.get_function("printf")
								.unwrap_or(self.module.add_function("printf", func_type));
							self.builder.build_call(printf_func, &[&s], "");
						}
						_ => panic!("type error, not a string"),
					}

					Value::Null
				}
				"itoa" => {
					if expr.args.len() != 1 {
						panic!("arity 1 expected");
					}
					let res = self.walk(&expr.args[0]);

					match res {
						Value::Numeric(f) => {
							let i8_pointer_type = self.context.i8_type().pointer_type(0);

							let func_type = llvm::FunctionType::new(
								i8_pointer_type,
								&[i8_pointer_type, i8_pointer_type, self.context.double_type()],
								true,
							);
							let sprintf = self
								.module
								.get_function("sprintf")
								.unwrap_or(self.module.add_function("sprintf", func_type));

							let arr_type = self.context.array_type(self.context.i8_type(), 100);

							let format_str = self.builder.build_global_string_ptr("%f", "");

							let arr = self.builder.build_alloca(arr_type, "");
							self.builder
								.build_call(sprintf, &[&arr, &format_str, &f], "");
							Value::String(arr)
						}
						_ => panic!("type error, not a string"),
					}
				}
				_ => {
					panic!("undefined function")
				}
			},
			_ => panic!("evaluation error"),
		}
	}

	fn visit_while(&mut self, expr: &expression::While) -> Value {
		self.context.create_basic_block("loop");
		let fun = self.builder.get_insert_block().get_parent();
		let loop_block = self.context.append_basic_block(&fun, "loop");
		self.builder.create_br(&loop_block);
		self.builder.position_builder_at_end(&loop_block);
		for stmt in &expr.body {
			self.walk(stmt);
		}
		self.builder.create_br(&loop_block);
		let after_loop_block = self.context.append_basic_block(&fun, "afterloop");
		self.builder.position_builder_at_end(&after_loop_block);
		Value::Null
	}

	fn visit_identifier(&mut self, _: &str) -> Value {
		todo!()
	}

	fn visit_string(&mut self, expr: &str) -> Value {
		let with_newlines = expr.to_string().replace("\\n", "\n");
		Value::String(
			self.builder
				.build_global_string_ptr(with_newlines.as_str(), ""),
		)
	}

	fn visit_bool(&mut self, _: &bool) -> Value {
		todo!()
	}

	fn visit_break(&mut self) -> Value {
		todo!()
	}

	fn visit_program(&mut self, program: parser::Program) -> Value {
		let void_t = self.context.void_type();
		let sum_type = llvm::FunctionType::new(void_t, &[], false);
		let sum_fun = self.module.add_function(MAIN_FUNCTION, sum_type);
		let block = self.context.append_basic_block(&sum_fun, "entry");
		self.builder.position_builder_at_end(&block);

		for stmt in program.body {
			self.walk(&stmt);
		}

		self.builder.build_ret_void();
		Value::Null
	}

	fn visit_func_decl(&mut self, _: &expression::FuncDecl) -> Value {
		todo!()
	}
}

impl Compile for Compiler {
	fn compile(&mut self) -> Result<(), Box<dyn Error>> {
		self.visit_program(self.program.clone());
		Ok(())
	}
}

impl Compiler {
	pub fn dump_ir(&self) {
		self.module.dump();
	}

	pub fn run(&self) {
		self.engine.call(MAIN_FUNCTION);
	}

	pub fn new(program: Program) -> Self {
		let context = llvm::Context::new();
		let module = llvm::Module::new("main", &context);
		let builder = llvm::Builder::new(&context);
		let engine = llvm::Engine::new(&module);

		Compiler {
			program,
			context,
			module,
			builder,
			engine,
		}
	}
}
