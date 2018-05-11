mod cd;
mod launch_bin;
mod env;

fn command_parser(mut command_vector : Vec<&str>) {
    use cd::cd;
    use launch_bin::launch_bin;
    use env::{env, setenv, unsetenv};

    match command_vector[0].trim() {
        "cd"        => cd(&command_vector),
        "env"       => env(),
        "setenv"    => setenv(&mut command_vector),
        "unsetenv"  => unsetenv(&mut command_vector),
        ""          => {},
        _           => launch_bin(&mut command_vector)
    }
}

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

fn clean_cmd(cmd: &mut String) -> String {
    if cmd.chars().last().unwrap() == '\n' {
        cmd.pop();
    }
    return cmd.trim().to_string();
}

fn get_command_vector(command: &mut String) {
    let all_commands = command.split(";").map(|x: &str| x.to_string()).collect::<Vec<String>>();
    for mut cmd in all_commands {
        {
            cmd = clean_cmd(&mut cmd);
            let cmd_vector = cmd.split(" ").collect::<Vec<&str>>();
            command_parser(cmd_vector);
        }
    }
    print_prompt();
}

fn main_loop() {
    let mut command = String::new();

    while std::io::stdin().read_line(&mut command).unwrap() > 0
    {
        get_command_vector(&mut command);
        command.clear();
    }
}

fn main() {
    print_prompt();
    main_loop();
    println!();
    return ;
}
