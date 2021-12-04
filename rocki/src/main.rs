use rocklang::compiler::Compile;
use rocklang::compiler::Compiler;
use rocklang::parser::Parse;
use rocklang::parser::Parser;
use rocklang::tokenizer::Tokenize;
use rocklang::tokenizer::Tokenizer;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::error::Error;

const HISTORY_FILENAME: &str = ".rocki_history";

trait Evaulate {
    fn evaluate(&self, line: &str) -> Result<std::string::String, Box<dyn Error>>;
}

struct Evaluator {}

impl Evaulate for Evaluator {
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

fn main() {
    let mut rl = Editor::<()>::new();
    let evaluator = Evaluator::new();

    let history_path = match home::home_dir() {
        Some(path) => Some(path.join(HISTORY_FILENAME)),
        None => None,
    };

    #[allow(unused_must_use)]
    {
        match history_path {
            Some(ref path) => {
                rl.load_history(&path);
            }
            None => (),
        }
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                evaluator.evaluate(&line).unwrap();
                evaluator.evaluate("print(\"\\n\")").unwrap();
                rl.add_history_entry(line.as_str());
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    match history_path {
        Some(path) => {
            rl.save_history(&path).unwrap();
        }
        None => (),
    }
}
