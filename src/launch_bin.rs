use std::process::Command;

pub fn launch_bin(command_vector: &mut Vec<&str>)
{
    let program_string = command_vector[0];
    let mut process = Command::new(program_string.trim());

    command_vector.remove(0);
    for val in command_vector {
        process.arg(val.trim());
    }
    let child = process.spawn().ok();
    if child.is_none() {
        println!("Couldn't launch process: {}", program_string)
    } else {
        child.unwrap().wait().unwrap();
    }
}
