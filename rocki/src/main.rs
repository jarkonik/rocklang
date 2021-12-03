use rocklang::compiler::Compile;
use rocklang::compiler::Compiler;
use rocklang::parser::Parse;
use rocklang::parser::Parser;
use rocklang::tokenizer::Tokenize;
use rocklang::tokenizer::Tokenizer;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::error::Error;

const HISTORY_PATH: &str = "~/.rocki_history";

fn evaluate_line(line: &str) -> Result<(), Box<dyn Error>> {
    let mut tokenizer = Tokenizer::new(line.to_string());
    let tokens = tokenizer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    let mut compiler = Compiler::new(ast);
    compiler.compile()?;

    Ok(())
}

fn main() {
    let mut rl = Editor::<()>::new();

    #[allow(unused_must_use)]
    {
        rl.load_history(HISTORY_PATH);
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                evaluate_line(&line).unwrap();
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
    rl.save_history(HISTORY_PATH).unwrap();
}
