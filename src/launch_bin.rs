
pub fn launch_bin(command_vector: &mut Vec<&str>)
{
    use std::process::Command;

    let mut process = Command::new(command_vector[0].trim());

    command_vector.remove(0);
    for val in command_vector {
        process.arg(val.trim());;
    }

    match process.spawn() {
        Err(e)  => panic!("Error : {}", e),
        _       => {}
    }
}
