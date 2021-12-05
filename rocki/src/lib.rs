use rocklang::compiler::Compile;
use rocklang::compiler::Compiler;
use rocklang::parser::Parse;
use rocklang::parser::Parser;
use rocklang::tokenizer::Tokenize;
use rocklang::tokenizer::Tokenizer;
use std::error::Error;

pub trait Evaluate {
	fn evaluate(&mut self, line: &str) -> Result<std::string::String, Box<dyn Error>>;
}

pub struct Evaluator {
	compiler: Compiler,
}

impl Evaluate for Evaluator {
	fn evaluate(&mut self, line: &str) -> Result<std::string::String, Box<dyn Error>> {
		let mut tokenizer = Tokenizer::new(line.to_string());
		let tokens = tokenizer.tokenize()?;

		let mut parser = Parser::new(tokens);
		let ast = parser.parse()?;

		let f = self.compiler.compile(ast)?;
		self.compiler.call(f);

		Ok(String::from(""))
	}
}

impl Evaluator {
	pub fn new() -> Self {
		let compiler = Compiler::new();

		Evaluator { compiler }
	}
}
