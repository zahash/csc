use crate::eval::*;

use rustyline::error::ReadlineError;

const LOGO: &'static str = r#"
 ██████ ███████  ██████
██      ██      ██     
██      ███████ ██     
██           ██ ██     
 ██████ ███████  ██████
"#;

pub fn run() -> anyhow::Result<()> {
    let expr = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    if !expr.trim().is_empty() {
        match eval(expr.as_str(), &mut State::new()) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{:?}", e),
        }
        return Ok(());
    }

    println!("{}", LOGO);
    println!(env!("CARGO_PKG_VERSION"));

    println!("To Quit, press CTRL-C or CTRL-D or type 'exit' or 'quit'");

    let mut state = State::new();
    let mut editor = rustyline::DefaultEditor::new().unwrap();

    loop {
        match editor.readline("> ").as_deref() {
            Ok("clear") | Ok("cls") => editor.clear_screen()?,
            Ok("exit") | Ok("quit") => break,
            Ok(line) => {
                if !line.is_empty() {
                    let _ = editor.add_history_entry(line);
                    match eval(line, &mut state) {
                        Ok(res) => println!(">>> {}", res),
                        Err(e) => eprintln!("!! {:?}", e),
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

    Ok(())
}
