extern crate regex;

#[derive(Debug)]
enum CommandOperator {
    AND,
    OR,
    ALWAYS,
    PIPE
}

#[derive(Debug)]
struct Command {
    command: Vec<String>,
    command_operator: CommandOperator
}

fn clean_cmd(mut cmd: String) -> String {
    match cmd.chars().last() {
        Some(c) => {if c == '\n' { cmd.pop(); }},
        _ => {}
    }
    return cmd.trim().to_string();
}

fn get_all_commands(command: &String) -> Vec<Command>{
    let mut commands: Vec<Command> = vec![];
    let re= regex::Regex::new(r";|&&|\|\||\|").unwrap();
    let mut caps = re.captures_iter(command);

    for mut part in re.split(command) {
        let cmd: String = clean_cmd(part.to_string());
        let cmd_vector = cmd.split(" ").map(|x: &str| x.to_string()).collect::<Vec<String>>();
        commands.push(Command {
            command: cmd_vector,
            command_operator:
            match caps.next() {
                Some(t) => {
                    let m = t.get(0).unwrap().as_str();
                    if m == ";" {
                        CommandOperator::ALWAYS
                    } else if m == "&&" {
                        CommandOperator::AND
                    } else if m == "||" {
                        CommandOperator::OR
                    } else if m == "|" {
                        CommandOperator::PIPE
                    } else {
                        CommandOperator::ALWAYS
                    }
                },
                _ => {
                    CommandOperator::ALWAYS
                }
            }
        });
    }
    return commands;
}

fn command_parser(mut command_vector : Vec<String>) -> i32 {
    use cd::cd;
    use launch_bin::launch_bin;
    use env::{env, setenv, unsetenv};

    match command_vector[0].trim() {
        "cd"        => return cd(&command_vector),
        "env"       => return env(),
        "setenv"    => return setenv(&mut command_vector),
        "unsetenv"  => return unsetenv(&mut command_vector),
        ""          => {return 0},
        _           => return launch_bin(&mut command_vector)
    }
}

pub fn handle_command(command: &mut String) {
    let mut status: i32 = 0;
    let all_commands: Vec<Command> = get_all_commands(command);
    let mut operator = CommandOperator::ALWAYS;

    for mut cmd in all_commands {
        match operator {
            CommandOperator::AND => if status != 0 { return } else { status = command_parser(cmd.command) },
            CommandOperator::OR  => if status == 0 { return } else { status = command_parser(cmd.command) },
            _ => { status = command_parser(cmd.command) }
        }
        operator = cmd.command_operator;
    }
}
