#[macro_use]
extern crate downcast_rs;
extern crate signal_hook;
extern crate regex;
extern crate core;

mod cd;
mod launch_bin;
mod env;
mod command_handler;
mod parser { pub mod parser; pub mod lexer; pub mod nodes; }
mod interpreter;

use interpreter::Interpreter;
use parser::parser::Parser;
use std::thread;
use signal_hook::iterator::Signals;
use std::error::Error;

fn print_prompt() {
    use std::io;
    use std::io::Write;
    use std::env;

    if let Ok(dir) = env::current_dir() {
        print!("{} > ", dir.display());
        io::stdout().flush().unwrap();
    } else {
        print!("> ");
        io::stdout().flush().unwrap();
    }
}



fn main_loop() {
    let mut command = String::new();
    let mut interpreter: Interpreter = Interpreter { parser: Parser::new() };

    while std::io::stdin().read_line(&mut command).unwrap() > 0 {
        let cmd = command.clone();
        interpreter.interpret(cmd);
        command.clear();
        print_prompt();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let signals = Signals::new(&[signal_hook::SIGINT])?;

    thread::spawn(move || {
        for _ in signals.forever() {
            println!();
            print_prompt();
        }
    });


    print_prompt();
    main_loop();
    println!();
    return Ok(());
}
