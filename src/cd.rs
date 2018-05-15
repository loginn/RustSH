use std::env;
use std::path;

fn change_directory(path: &path::PathBuf) -> i32 {
    match env::set_current_dir(path) {
        Err(err) => {println!("Error : {}, {:?}", err, path); return 1},
        Ok(_) => {return 0}
    }
}

fn build_new_path(command_path: &str) -> Option<path::PathBuf> {
    let mut desired_path = path::PathBuf::new();
    let user_path = path::PathBuf::from(command_path);
    if user_path.is_relative() {
        let current_path = env::current_dir();
        match current_path {
            Ok(p) => {
                desired_path.push(p);
                desired_path.push(user_path);
            },
            Err(e) => {
                println!("Error : {}", e);
                return None;
            }
        }
    } else {
        desired_path.push(user_path);
    }
    return Some(desired_path)
}

pub fn cd (command_vector: &Vec<String> ) -> i32 {
    if command_vector.len() == 2 {
        match build_new_path(&command_vector[1]) {
            Some(n) => return change_directory(&n),
            None => return 1
        }
    } else if command_vector.len() == 1 {
        match env::home_dir() {
            Some(n) => change_directory(&n),
            None => {println!("No known home directory"); return 1}
        }
    } else {
        println!("Error : Invalid path");
        return 1
    }
}
