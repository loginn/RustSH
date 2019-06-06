use std::env;
use std::io;
use std::io::{Write};
use command_handler::CommandResult;

pub fn env() -> CommandResult
{
    let _environement = env::vars();
//    match stdout {
//        None => {
//            for x in environement {
//                print!("{}={}\n", x.0, x.1);
//            }
//        },
//        Some(mut stdout) => {
//            for x in environement {
//                stdout.write_all(format!("{}={}\n", x.0, x.1).as_bytes());
//            }
//        }
//    }
    return CommandResult { child: None, status: 0 };
}

pub fn setenv(command_vector : &mut Vec<String>) -> CommandResult
{
    if command_vector.len() > 1 {
        let values = command_vector[1].split("=").collect::<Vec<&str>>();
        if values.len() > 1 {
            env::set_var(values[0], values[1]);
            return CommandResult { child: None, status: 0 };
        }
        return CommandResult { child: None, status: 1 };
    } else {
        match writeln!(&mut io::stderr(), "Invalid pair") {
            Err(e)  => println!("{}", e),
            _       => {}
        }
        return CommandResult { child: None, status: 1 };
    }
}

pub fn unsetenv(command_vector : &mut Vec<String>) -> CommandResult
{
    if command_vector.len() > 1 {
        env::remove_var(command_vector[1].trim());
        return CommandResult { child: None, status: 0 };
        } else {
            match writeln!(&mut io::stderr(), "No input var") {
                Err(e)  => println!("{}", e),
                _       => {}
        }
        return CommandResult { child: None, status: 1 };
    }
}
