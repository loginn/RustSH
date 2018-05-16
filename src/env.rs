use std::env;
use std::io;
use std::io::Write;
use command_handler::CommandResult;

pub fn env() -> CommandResult
{
    let environement = env::vars();

    for x in environement {
        print!("{}={}\n", x.0, x.1);
    }
    return CommandResult {status: 0, output: None};
}

pub fn setenv(command_vector : &mut Vec<String>) -> CommandResult
{
    if command_vector.len() > 1 {
        let values = command_vector[1].split("=").collect::<Vec<&str>>();
        if values.len() > 1 {
            env::set_var(values[0], values[1]);
            return CommandResult {status: 0, output: None};
        }
        return CommandResult {status: 1, output: None};
    } else {
        match writeln!(&mut io::stderr(), "Invalid pair") {
            Err(e)  => println!("{}", e),
            _       => {}
        }
        return CommandResult {status: 1, output: None};
    }
}

pub fn unsetenv(command_vector : &mut Vec<String>) -> CommandResult
{
    if command_vector.len() > 1 {
        env::remove_var(command_vector[1].trim());
        return CommandResult {status: 0, output: None};
        } else {
            match writeln!(&mut io::stderr(), "No input var") {
                Err(e)  => println!("{}", e),
                _       => {}
        }
        return CommandResult {status: 1, output: None};
    }
}
