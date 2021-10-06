mod evaluator;
mod expression;
mod parser;
mod token;
mod tokenizer;
mod value;
mod visitor;

use evaluator::Evaluate;
use parser::Parse;
use std::{env, fs};
use tokenizer::Tokenize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No input file provided");
    }
    let filename = &args[1];

    let source = fs::read_to_string(filename).expect("Error reading input file");

    let mut tokenizer = tokenizer::Tokenizer::new(source);
    let tokens = tokenizer.tokenize();

    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();

    // let json = serde_json::to_string_pretty(&ast).unwrap();
    // println!("{}", json);

    let mut evaluator = evaluator::Evaluator::new(ast);
    evaluator.evaluate();
}
