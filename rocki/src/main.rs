use rustyline::error::ReadlineError;
use rustyline::Editor;

const HISTORY_PATH: &str = "~/.rocki_history";

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
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
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
