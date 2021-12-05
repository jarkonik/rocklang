use rocki::{Evaluate, Evaluator};
use rustyline::error::ReadlineError;
use rustyline::Editor;

const HISTORY_FILENAME: &str = ".rocki_history";

fn main() {
    let mut rl = Editor::<()>::new();
    let mut evaluator = Evaluator::new();

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
