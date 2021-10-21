use crate::expression;
use crate::llvm;
use crate::llvm::PassManager;
use crate::parser;
use crate::parser::Program;
use crate::visitor::Visitor;
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;

const MAIN_FUNCTION: &str = "__main__";

struct Frame {
	env: HashMap<String, Value>,
}

impl Frame {
	pub fn new() -> Self {
		Frame {
			env: HashMap::new(),
		}
	}

	pub fn get(&self, literal: &str) -> Option<&Value> {
		self.env.get(literal)
	}

	pub fn set(&mut self, literal: &str, val: Value) {
		self.env.insert(literal.to_string(), val);
	}
}

impl Default for Frame {
	fn default() -> Self {
		Frame::new()
	}
}

pub trait Compile {
	fn compile(&mut self) -> Result<(), Box<dyn Error>>;
}

pub struct Compiler {
	program: Program,
	engine: llvm::Engine,
	context: llvm::Context,
	module: llvm::Module,
	builder: llvm::Builder,
	fpm: PassManager,
	opt: bool,
	stack: Vec<Frame>,
}

#[derive(Debug, Clone)]
enum Value {
	Null,
	String(llvm::Value),
	GlobalString(llvm::Value),
	Numeric(llvm::Value),
	Ptr(llvm::Value),
	Bool(llvm::Value),
	Function(llvm::Value),
}

impl Visitor<Value> for Compiler {
	fn visit_binary(&mut self, expr: &expression::Binary) -> Value {
		match expr.operator {
			expression::Operator::Plus => {
				let l = match self.walk(&expr.left) {
					Value::Ptr(p) => p,
					Value::Numeric(n) => n,
					_ => panic!("panic"),
				};

				let r = match self.walk(&expr.right) {
					Value::Ptr(p) => p,
					Value::Numeric(n) => n,
					_ => panic!("panic"),
				};

				Value::Numeric(self.builder.build_fadd(l, r, ""))
			}
			expression::Operator::LessOrEqual => {
				let l = match self.walk(&expr.left) {
					Value::Ptr(p) => p,
					Value::Numeric(n) => n,
					_ => panic!("panic"),
				};

				let r = match self.walk(&expr.right) {
					Value::Ptr(p) => p,
					Value::Numeric(n) => n,
					_ => panic!("panic"),
				};

				Value::Bool(self.builder.build_fcmp(l, r, llvm::Cmp::LessOrEqual, ""))
			}
			expression::Operator::Less => {
				let l = match self.walk(&expr.left) {
					Value::Ptr(p) => p,
					Value::Numeric(n) => n,
					_ => panic!("panic"),
				};

				let r = match self.walk(&expr.right) {
					Value::Ptr(p) => p,
					Value::Numeric(n) => n,
					_ => panic!("panic"),
				};

				Value::Bool(self.builder.build_fcmp(l, r, llvm::Cmp::Less, ""))
			}
			_ => todo!(),
		}
	}

	fn visit_numeric(&mut self, f: &f64) -> Value {
		Value::Numeric(self.context.const_double(*f))
	}

	fn visit_conditional(&mut self, expr: &expression::Conditional) -> Value {
		let fun = self.builder.get_insert_block().get_parent();

		let then_block = self.context.append_basic_block(&fun, "then");
		let else_block = self.context.append_basic_block(&fun, "else");
		let after_if_block = self.context.append_basic_block(&fun, "afterif");

		let predicate = self.walk(&expr.predicate);
		match predicate {
			Value::Bool(b) => {
				self.builder.build_cond_br(&b, &then_block, &else_block);
				self.builder.position_builder_at_end(&then_block);
				for stmt in &expr.body {
					self.walk(stmt);
				}
				self.builder.create_br(&after_if_block);

				self.builder.position_builder_at_end(&else_block);
				for stmt in &expr.else_body {
					self.walk(stmt);
				}
				self.builder.create_br(&after_if_block);

				self.builder.position_builder_at_end(&after_if_block);

				Value::Null
			}
			_ => panic!("type error"),
		}
	}

	fn visit_assignment(&mut self, expr: &expression::Assignment) -> Value {
		let literal = match &*expr.left {
			expression::Expression::Identifier(literal) => literal,
			_ => panic!("panic"),
		};

		let val = self.walk(&expr.right);

		match val {
			Value::Numeric(n) => {
				let var = self.get_var(literal);

				match var {
					Some(v) => match v {
						Value::Ptr(p) => {
							self.builder.create_store(n, &p);
						}
						_ => panic!("panic"),
					},
					_ => {
						let alloca = self.builder.build_alloca(self.context.double_type(), "");
						self.builder.create_store(n, &alloca);
						self.set_var(literal, Value::Ptr(alloca));
					}
				};
				Value::Null
			}
			Value::Function(f) => {
				let var = self.get_var(literal);

				match var {
					Some(v) => match v {
						Value::Ptr(p) => {
							self.builder.create_store(f, &p);
						}
						_ => panic!("panic"),
					},
					_ => {
						let fun_type = self.context.function_type(
							self.context.void_type(),
							&[self.context.double_type()],
							false,
						);
						let alloca = self.builder.build_alloca(fun_type.pointer_type(0), "");
						self.builder.create_store(f, &alloca);
						self.set_var(literal, Value::Function(alloca));
					}
				};

				Value::Null
			}
			_ => panic!("type error"),
		}
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
						Value::GlobalString(s) => {
							let void_type = self.context.void_type();
							let i8_pointer_type = self.context.i8_type().pointer_type(0);
							let func_type =
								self.context
									.function_type(void_type, &[i8_pointer_type], false);
							let printf_func = self
								.module
								.get_function("printf")
								.unwrap_or(self.module.add_function("printf", func_type));
							let p = self.builder.build_bitcast(&s, i8_pointer_type, "");
							self.builder.build_call(&printf_func, &[p], "");
						}
						Value::String(s) => {
							let void_type = self.context.void_type();
							let i8_pointer_type = self.context.i8_type().pointer_type(0);
							let func_type =
								self.context
									.function_type(void_type, &[i8_pointer_type], false);
							let printf_func = self
								.module
								.get_function("printf")
								.unwrap_or(self.module.add_function("printf", func_type));
							let p = self.builder.build_bitcast(&s, i8_pointer_type, "");
							self.builder.build_call(&printf_func, &[p], "");
							self.builder.build_free(s);
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

							let func_type = self.context.function_type(
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

							// 							/// CreateEntryBlockAlloca - Create an alloca instruction in the entry block of
							// /// the function.  This is used for mutable variables etc.
							// static AllocaInst *CreateEntryBlockAlloca(Function *TheFunction,
							//                                           const std::string &VarName) {
							//   IRBuilder<> TmpB(&TheFunction->getEntryBlock(),
							//                  TheFunction->getEntryBlock().begin());
							//   return TmpB.CreateAlloca(Type::getDoubleTy(TheContext), 0,
							//                            VarName.c_str());
							let arr = self.builder.build_malloc(arr_type, "");
							let p = self.builder.build_bitcast(&arr, i8_pointer_type, "");
							self.builder.build_call(&sprintf, &[p, format_str, f], "");

							Value::String(arr)
						}
						_ => panic!("type error, not a string"),
					}
				}
				_ => match &self.get_var(literal) {
					Some(Value::Function(f)) => {
						let args: Vec<llvm::Value> = expr
							.args
							.iter()
							.map(|arg| match self.walk(arg) {
								Value::Numeric(n) => n,
								_ => todo!(),
							})
							.collect();

						self.builder
							.build_call(&self.builder.build_load(f, ""), &args, "");
						Value::Null
					}
					_ => panic!("{} undefined", literal),
				},
			},
			_ => panic!("evaluation error"),
		}
	}

	fn visit_while(&mut self, expr: &expression::While) -> Value {
		let predicate = self.walk(&expr.predicate);
		match predicate {
			Value::Bool(b) => {
				let fun = self.builder.get_insert_block().get_parent();

				let loop_block = self.context.append_basic_block(&fun, "loop");
				let after_loop_block = self.context.append_basic_block(&fun, "afterloop");

				self.builder
					.build_cond_br(&b, &loop_block, &after_loop_block);

				self.builder.position_builder_at_end(&loop_block);

				for stmt in &expr.body {
					self.walk(stmt);
				}
				let term_pred = self.walk(&expr.predicate);

				match term_pred {
					Value::Bool(b) => {
						self.builder
							.build_cond_br(&b, &loop_block, &after_loop_block);
					}
					_ => panic!("type error"),
				}

				self.builder.position_builder_at_end(&after_loop_block);
			}
			_ => panic!("type error"),
		}

		Value::Null
	}

	fn visit_identifier(&mut self, expr: &str) -> Value {
		match &self.get_var(expr) {
			Some(Value::Ptr(n)) => Value::Numeric(self.builder.build_load(n, "")),
			Some(Value::Numeric(n)) => Value::Numeric(n.clone()),
			_ => Value::Numeric(self.context.const_float(0.0)),
		}
	}

	fn visit_string(&mut self, expr: &str) -> Value {
		let with_newlines = expr.to_string().replace("\\n", "\n");
		Value::GlobalString(
			self.builder
				.build_global_string_ptr(with_newlines.as_str(), ""),
		)
	}

	fn visit_bool(&mut self, expr: &bool) -> Value {
		Value::Bool(self.context.const_bool(*expr))
	}

	fn visit_break(&mut self) -> Value {
		todo!()
	}

	fn visit_program(&mut self, program: parser::Program) -> Value {
		let void_t = self.context.void_type();
		let sum_type = self.context.function_type(void_t, &[], false);
		let sum_fun = self.module.add_function(MAIN_FUNCTION, sum_type);
		let block = self.context.append_basic_block(&sum_fun, "entry");
		self.builder.position_builder_at_end(&block);

		for stmt in program.body {
			self.walk(&stmt);
		}

		self.builder.build_ret_void();

		if self.opt {
			self.fpm.run(&sum_fun);
		}

		Value::Null
	}

	fn visit_func_decl(&mut self, expr: &expression::FuncDecl) -> Value {
		let void_t = self.context.void_type();
		let fun_type = self
			.context
			.function_type(void_t, &[self.context.double_type()], false);

		let curr = self.builder.get_insert_block();

		let fun = self.module.add_function("fun", fun_type);
		let block = self.context.append_basic_block(&fun, "entry");
		self.builder.position_builder_at_end(&block);

		self.stack.push(Frame::default());

		for (i, param) in expr.params.iter().enumerate() {
			self.set_var(param, Value::Numeric(fun.get_param(i.try_into().unwrap())))
		}

		for stmt in expr.body.clone() {
			self.walk(&stmt);
		}

		self.stack.pop();

		self.builder.build_ret_void();

		self.builder.position_builder_at_end(&curr);

		if self.opt {
			self.fpm.run(&fun);
		}

		Value::Function(fun)
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

	fn set_var(&mut self, literal: &str, val: Value) {
		self.stack.last_mut().unwrap().set(literal, val);
	}

	fn get_var(&mut self, literal: &str) -> Option<Value> {
		match self.stack.last_mut().unwrap().get(literal) {
			Some(v) => Some((*v).clone()),
			_ => None,
		}
	}

	pub fn run(&self) {
		self.engine.call(MAIN_FUNCTION);
	}

	pub fn no_opt(&mut self) {
		self.opt = false;
	}

	pub fn new(program: Program) -> Self {
		let context = llvm::Context::new();
		let module = llvm::Module::new("main", &context);
		let builder = llvm::Builder::new(&context);
		let engine = llvm::Engine::new(&module);

		let fpm = llvm::PassManager::new(&module);

		Compiler {
			program,
			context,
			module,
			builder,
			engine,
			stack: vec![Frame::new()],
			fpm,
			opt: true,
		}
	}
}
