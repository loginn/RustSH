use std::io;
mod cd;
mod launch_bin;

fn main_loop() {
    use cd::cd;
    use launch_bin::launch_bin;

    let mut command = String::new();
    while io::stdin().read_line(&mut command).unwrap() > 0
    {
        {
            let mut command_vector = command.split(" ").collect::<Vec<&str>>();

            match command_vector[0].trim() {
                "cd" => cd(&command_vector),
                _ => launch_bin(&mut command_vector)
            }
        }
        command.clear();
    }
}

fn main() {
    main_loop();
    return ;
}
