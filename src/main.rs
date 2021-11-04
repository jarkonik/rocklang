use rocklang::compiler::{Compile, Compiler};
use rocklang::parser::{Parse, Parser};
use rocklang::tokenizer::{Tokenize, Tokenizer};
use std::error::Error;
use std::fmt;
use std::{env, fs};

#[derive(Clone, Debug, PartialEq)]
pub struct InputError {}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Input error")
    }
}

impl Error for InputError {}

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(InputError {}));
    }
    let filename = &args[1];

    let mut dump_ir = false;
    let mut no_opt = false;
    let mut dump_ast = false;

    for arg in std::env::args() {
        match arg.as_str() {
            "--ir" => dump_ir = true,
            "--no-opt" => no_opt = true,
            "--ast" => dump_ast = true,
            _ => (),
        }
    }

    let source = fs::read_to_string(filename).expect("Error reading input file");

    let mut tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    if dump_ast {
        let json = serde_json::to_string_pretty(&ast).unwrap();
        println!("{}", json);
        return Ok(());
    }

    // let mut evaluator = Evaluator::new(ast);
    // evaluator.evaluate();

    let mut compiler = Compiler::new(ast);
    if no_opt {
        compiler.no_opt();
    }

    compiler.compile()?;
    if dump_ir {
        compiler.dump_ir();
    } else {
        compiler.run();
    }

    Ok(())
}
