use crate::eval::*;

use clap::Parser;
use rustyline::error::ReadlineError;

const LOGO: &'static str = r#"
 ██████ ███████  ██████
██      ██      ██     
██      ███████ ██     
██           ██ ██     
 ██████ ███████  ██████
"#;

/// CSC
#[derive(Parser)]
struct Cli {
    /// run one off computations instead of launching the prompt
    expr: Option<Vec<String>>,
}

pub fn run() {
    if let Some(expr) = Cli::parse().expr {
        let expr = expr.join(" ");
        match eval(expr.as_str(), &mut State::new()) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{:?}", e),
        }
        return;
    }

    println!("{}", LOGO);
    println!(env!("CARGO_PKG_VERSION"));

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
