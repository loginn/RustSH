use std::process::{Command};
use command_handler::{CommandResult};

fn add_args(process: &mut Command, command_vector: &Vec<String>) {
    for val in command_vector.iter().enumerate() {
        if val.0 > 0 {
            process.arg(val.1.trim());
        }
    }
}

fn create_process(command_vector: &mut Vec<String>) -> Option<Command> {
    let mut process = Command::new(&command_vector[0].trim());

    add_args(&mut process, command_vector);
    return Some(process)
}

fn create_child_process(command_vector: &mut Vec<String>) -> Option<Command> {
    return create_process(command_vector)
}

pub fn launch_bin(command_vector: &mut Vec<String>) -> CommandResult {
    let mut child = match create_child_process(command_vector) {
        None => { return CommandResult {child: None, status: 1}; },
        Some(child) => child
    };

    let status = child.spawn().unwrap().wait().unwrap().code().unwrap();
    let return_value = CommandResult {
        status,
        child: Some(child),
    };

    return return_value
}
