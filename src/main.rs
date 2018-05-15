mod cd;
mod launch_bin;
mod env;
mod command_handler;

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

    while std::io::stdin().read_line(&mut command).unwrap() > 0 {
        command_handler::handle_command(&mut command);
        command.clear();
        print_prompt();
    }
}

fn main() {
    print_prompt();
    main_loop();
    println!();
    return ;
}
