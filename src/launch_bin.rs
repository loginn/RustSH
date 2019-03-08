use std::process::{Command, Stdio};
use command_handler::{CommandResult};
use std::error::Error;


fn add_args(process: &mut Command, command_vector: &Vec<String>) {
    for val in command_vector.iter().enumerate() {
        if val.0 > 0 {
            process.arg(val.1.trim());
        }
    }
}

fn create_process(command_vector: &mut Vec<String>, stdin: Option<Stdio>, stdout: Option<Stdio>) -> Option<Command> {
    let mut process = Command::new(&command_vector[0].trim());
    match stdin {
        None => {},
        Some(stdin) => { process.stdin(stdin);},
    }

    match stdout {
        None => {},
        Some(stdout) => { process.stdout(stdout);},
    }

    add_args(&mut process, command_vector);
    return Some(process)
}

fn create_child_process(command_vector: &mut Vec<String>, stdin: Option<Stdio>, stdout: Option<Stdio>) -> Option<Command> {
    return create_process(command_vector, stdin, stdout)
}

fn run_child(child: &mut Command, command_vector: &Vec<String>) -> i32 {
    let status : i32 = match child.spawn() {
        Err(_) => {
            println!("RustSH: {} : command not found", command_vector[0]);
            1
        },
        Ok(mut spawn) => {
            match spawn.wait() {
                Err(e) => {
                    println!("RustSH: {}", e.description());
                    1
                },
                Ok(wait) => {
                    match wait.code() {
                        None => 1,
                        Some(code) => code
                    }
                }
            }
        }
    };
    return status
}

pub fn launch_bin(command_vector: &mut Vec<String>, stdin: Option<Stdio>, stdout: Option<Stdio>) -> CommandResult {
    let mut child = match create_child_process(command_vector, stdin, stdout) {
        None => { return CommandResult {child: None, status: 1}; },
        Some(child) => child
    };

    let status: i32 = run_child(&mut child, command_vector);
    return CommandResult { child: Some(child), status }
}
