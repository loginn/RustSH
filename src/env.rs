use std::env;
use std::io;
use std::io::Write;

pub fn env() -> i32
{
    let environement = env::vars();

    for x in environement {
        print!("{}={}\n", x.0, x.1);
    }
    return 0
}

pub fn setenv(command_vector : &mut Vec<String>) -> i32
{
    if command_vector.len() > 1 {
        let values = command_vector[1].split("=").collect::<Vec<&str>>();
        if values.len() > 1 {
            env::set_var(values[0], values[1]);
            return 0;
        }
        return 1;
    } else {
        match writeln!(&mut io::stderr(), "Invalid pair") {
            Err(e)  => println!("{}", e),
            _       => {}
        }
        return 1;
    }
}

pub fn unsetenv(command_vector : &mut Vec<String>) -> i32
{
    if command_vector.len() > 1 {
        env::remove_var(command_vector[1].trim());
        return 0;
        } else {
            match writeln!(&mut io::stderr(), "No input var") {
                Err(e)  => println!("{}", e),
                _       => {}
        }
        return 1;
    }
}
