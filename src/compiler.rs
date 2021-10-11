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

impl Visitor<()> for Compiler {
	fn visit_binary(&mut self, _: &expression::Binary) {
		todo!()
	}

	fn visit_numeric(&mut self, _: &f64) {
		todo!()
	}

	fn visit_conditional(&mut self, _: &expression::Conditional) {
		todo!()
	}

	fn visit_assignment(&mut self, _: &expression::Assignment) {
		todo!()
	}

	fn visit_unary(&mut self, _: &expression::Unary) {
		todo!()
	}

	fn visit_grouping(&mut self, _: &expression::Expression) {
		todo!()
	}

	fn visit_func_call(&mut self, _: &expression::FuncCall) {}

	fn visit_while(&mut self, _: &expression::While) {
		todo!()
	}

	fn visit_identifier(&mut self, _: &str) {
		todo!()
	}

	fn visit_string(&mut self, _: &str) {
		todo!()
	}

	fn visit_bool(&mut self, _: &bool) {
		todo!()
	}

	fn visit_break(&mut self) {
		todo!()
	}

	fn visit_program(&mut self, program: parser::Program) {
		for stmt in program.body {
			self.walk(&stmt);
		}
	}

	fn visit_func_decl(&mut self, _: &expression::FuncDecl) {
		todo!()
	}
}

impl Compile for Compiler {
	fn compile(&mut self) -> Result<(), Box<dyn Error>> {
		self.create_sum_fn()?;
		self.visit_program(self.program.clone());
		self.engine.call(MAIN_FUNCTION);
		Ok(())
	}
}

impl Compiler {
	fn create_sum_fn(&self) -> Result<(), Box<dyn Error>> {
		let i64t = self.context.i64_type();
		let sum_type = llvm::FunctionType::new(
			i64t,
			&[
				self.context.i64_type(),
				self.context.i64_type(),
				self.context.i64_type(),
			],
			false,
		);
		let sum_fun = self.module.add_function(MAIN_FUNCTION, sum_type);
		let block = self.context.append_basic_block(&sum_fun, "entry");
		self.builder.position_builder_at_end(block);

		let hello_world_str = self.builder.build_global_string_ptr("hello world\n", "");
		let void_type = self.context.void_type();
		let i8_pointer_type = self.context.i8_type().pointer_type(0);
		let func_type = llvm::FunctionType::new(void_type, &[i8_pointer_type], false);
		let log_func = self.module.add_function("printf", func_type);
		self.builder.build_call(log_func, &[hello_world_str], "");

		// let x = sum_fun.get_param(0);
		// let y = sum_fun.get_param(1);
		// let z = sum_fun.get_param(2);

		// let sum = self.builder.build_add(x, y, "");
		// let sum = self.builder.build_add(sum, z, "");

		self.builder.build_ret_void();

		Ok(())
	}

	pub fn dump_ir(&self) {
		self.module.dump();
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
