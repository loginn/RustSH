use std::process::Command;

pub fn launch_bin(command_vector: &mut Vec<&str>)
{
    let mut process = Command::new(command_vector[0].trim());

    command_vector.remove(0);
    for val in command_vector {
        process.arg(val.trim());;
    }
    let child = process.spawn().ok();
    if child.is_none() {
        print!("Couldn't launch process")
    } else {
        child.unwrap().wait().unwrap();
    }
}
