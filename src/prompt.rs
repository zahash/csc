use crate::eval::*;

use rustyline::error::ReadlineError;

pub fn run() {
    println!("***** Scientific Calcualtor *****");

    let mut state = State::new();
    let mut rl = rustyline::DefaultEditor::new().unwrap();

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                if !line.is_empty() {
                    let _ = rl.add_history_entry(line.as_str());
                    match eval(line.as_str(), &mut state) {
                        Ok(res) => println!("{}", res),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
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
}
