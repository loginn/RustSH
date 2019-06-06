use std::process::{Command, Stdio, Child};


fn add_args(process: &mut Command, command_vector: &Vec<String>) {
    for val in command_vector.iter().enumerate() {
        if val.0 > 0 {
            process.arg(val.1.trim());
        }
    }
}

fn create_process(command_vector: &mut Vec<String>, stdin: Option<Stdio>, stdout: Option<Stdio>) -> Command {
    let mut process = Command::new(&command_vector[0].trim());
    match stdin {
        Some(sin) => {
            process.stdin(sin);
        },
        None => {}
    }

    match stdout {
        Some(sout) => {
            process.stdout(sout);
        },
        None => {}
    }

    add_args(&mut process, command_vector);
    return process
}

fn spawn_child(child: &mut Command, command_vector: &Vec<String>) -> Option<Child> {
    let spawn : Child = match child.spawn() {
        Err(e) => {
            eprintln!("RustSH: {} : command not found\n error : {}", command_vector[0], e);
            return None;
        },
        Ok(spawn) => { spawn }
    };
    return Some(spawn);
}

pub fn launch_bin(command_vector: &mut Vec<String>, stdin: Option<Stdio>, stdout: Option<Stdio>) -> Option<Child> {
    let mut process = create_process(command_vector, stdin, stdout);
    return spawn_child(&mut process, command_vector)

}
