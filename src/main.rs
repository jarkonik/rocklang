use rocklang::evaluator::{Evaluate, Evaluator};
use rocklang::parser::{Parse, Parser};
use rocklang::tokenizer::{Tokenize, Tokenizer};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No input file provided");
    }
    let filename = &args[1];

    let source = fs::read_to_string(filename).expect("Error reading input file");

    let mut tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    // let json = serde_json::to_string_pretty(&ast).unwrap();
    // println!("{}", json);

    let mut evaluator = Evaluator::new(ast);
    evaluator.evaluate();
}
