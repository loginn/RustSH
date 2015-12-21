use std::env;
use std::path;

fn change_directory(path: path::PathBuf) {
    match env::set_current_dir(&path) {
        Err(err) => println!("Error : {}", err),
        _ => {}
    }
}

pub fn cd (command_vector: &Vec<&str> ) {
    if command_vector.len() == 2 {
        let mut new_path = path::PathBuf::new();
        new_path.push(command_vector[1]);
        change_directory(new_path);
    } else if command_vector.len() == 1 {
        match env::home_dir() {
            Some(n) => change_directory(n),
            None => println!("No known home directory")
        }
    } else {
        println!("Error : Invalid path");
    }
}
