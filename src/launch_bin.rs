use std::process::{Command, Stdio, Child};
use std::io::prelude::*;
use command_handler::{CommandResult, CommandOperator};
use std::io;

fn add_args(process: &mut Command, command_vector: &Vec<String>) {
    for val in command_vector.iter().enumerate() {
        if val.0 > 0 {
            process.arg(val.1.trim());
        }
    }
}

fn create_process(command_vector: &mut Vec<String>, operator: &CommandOperator, input: &Option<String>) -> io::Result<Child> {
    let mut process = Command::new(&command_vector[0].trim());

    if input.is_some() {
        process.stdin(Stdio::piped());
    }

    match *operator {
        CommandOperator::PIPE  => {
            process.stdout(Stdio::piped());
        },
        CommandOperator::RIGHT => {
            process.stdout(Stdio::piped());
        }
        _ => {}
    }

    add_args(&mut process, command_vector);
    return process.spawn()
}

pub fn launch_bin(command_vector: &mut Vec<String>, operator: &CommandOperator, input: &Option<String>) -> CommandResult {

    let mut child = match create_process(command_vector, operator, &input) {
        Err(_e) => {
            return CommandResult {status: 1, output: None};
        },
        Ok(child) => child
    };

    match child.stdin {
        Some(ref mut stdin) => {
            match input {
                &Some(ref inp) => {
                    match stdin.write_all(inp.as_bytes()) {
                        Err(ref _why) => {},
                        Ok(ref _child) => {},
                    }
                },
                &None => {}
            }
        },
        None => {}
    }

    let status = child.wait().unwrap().code().unwrap();
    let mut output = String::new();

    match child.stdout {
        Some(mut out) => { out.read_to_string(&mut output).unwrap(); }
        None => { return CommandResult {status, output: None }; }
    }

    let return_value = CommandResult {
        status,
        output: Some(output)
    };
    return return_value
}
