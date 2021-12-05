use rocklang::compiler::Compile;
use rocklang::compiler::Compiler;
use rocklang::parser::Parse;
use rocklang::parser::Parser;
use rocklang::tokenizer::Tokenize;
use rocklang::tokenizer::Tokenizer;
use std::error::Error;

pub trait Evaluate {
	fn evaluate(&self, line: &str) -> Result<std::string::String, Box<dyn Error>>;
}

pub struct Evaluator {}

impl Evaluate for Evaluator {
	fn evaluate(&self, line: &str) -> Result<std::string::String, Box<dyn Error>> {
		let mut tokenizer = Tokenizer::new(line.to_string());
		let tokens = tokenizer.tokenize()?;

		let mut parser = Parser::new(tokens);
		let ast = parser.parse()?;

		let mut compiler = Compiler::new(ast);
		compiler.compile()?;
		compiler.run();

		Ok(String::from(""))
	}
}

impl Evaluator {
	pub fn new() -> Self {
		Evaluator {}
	}
}
