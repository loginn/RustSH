use std::process::Command;

pub fn launch_bin(command_vector: &mut Vec<String>) -> i32 {
    let mut process = Command::new(&command_vector[0].trim());

    command_vector.remove(0);
    for val in command_vector {
        process.arg(val.trim());
    }
    let child = process.spawn().ok();
    if child.is_none() {
        println!("Couldn't launch process: {:?}", process);
        return 1;
    } else {
        return child.unwrap().wait().unwrap().code().unwrap();
    }
}
