use std::env;
use std::io;
use std::io::Write;

pub fn env()
{
    let environement = env::vars();

    for x in environement {
        print!("{}={}\n", x.0, x.1);
    }
}

pub fn setenv(command_vector : &mut Vec<&str>)
{
    if command_vector.len() > 1 {
        let values = command_vector[1].split("=").collect::<Vec<&str>>();
        if values.len() > 1 {
            env::set_var(values[0], values[1]);
        }
    } else {
        match writeln!(&mut io::stderr(), "Invalid pair") {
            Err(e)  => println!("{}", e),
            _       => {}
        }
    }
}

pub fn unsetenv(command_vector : &mut Vec<&str>)
{
    if command_vector.len() > 1 {
        env::remove_var(command_vector[1].trim());
        } else {
            match writeln!(&mut io::stderr(), "No input var") {
                Err(e)  => println!("{}", e),
                _       => {}
        }
    }
}
