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
        _           => launch_bin(&mut command_vector)
    }
}

fn main_loop() {
    use std::io;
    use std::io::Write;
    use std::env;

    let mut command = String::new();
    let dir = env::current_dir().unwrap();
    print!("{} > ", dir.display());
    io::stdout().flush().unwrap();
    while io::stdin().read_line(&mut command).unwrap() > 0
    {
        {
            command.pop();
            let command_vector = command.split(" ").collect::<Vec<&str>>();
            command_parser(command_vector);
        }
        command.clear();
        let dir = env::current_dir().unwrap();
        print!("{} > ", dir.display());
        io::stdout().flush().unwrap();
    }
}

fn main() {
    main_loop();
    return ;
}
