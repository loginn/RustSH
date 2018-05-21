extern crate regex;

#[derive(Debug)]
pub enum CommandOperator {
    AND,
    OR,
    NONE,
    PIPE,
    RIGHT,
    LEFT
}

#[derive(Debug)]
pub struct CommandResult {
    pub status: i32,
    pub output: Option<String>
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
    let re= regex::Regex::new(r";|&&|\|\||\||>|<").unwrap();
    let mut caps = re.captures_iter(command);

    for mut part in re.split(command) {
        let cmd: String = clean_cmd(part.to_string());
        let cmd_vector = cmd.split(" ").map(|x: &str| x.to_string()).collect::<Vec<String>>();
        let op = match caps.next() {
            Some(t) => {t.get(0).unwrap().as_str()},
            None => ""
            };


        commands.push(Command {
            command: cmd_vector,
            command_operator:
            match op {
                ";" => CommandOperator::NONE,
                "&&" => CommandOperator::AND,
                "||" => CommandOperator::OR,
                "|" => CommandOperator::PIPE,
                ">" => CommandOperator::RIGHT,
                "<" => CommandOperator::LEFT,
                _ => CommandOperator::NONE
            }
        });
    }
    return commands;
}

fn command_parser(mut command_vector : &mut Vec<String>, operator: &CommandOperator, input: &Option<String>) -> CommandResult {
    use cd::cd;
    use launch_bin::launch_bin;
    use env::{env, setenv, unsetenv};

    match command_vector[0].trim() {
        "cd"        => return cd(&command_vector),
        "env"       => return env(),
        "setenv"    => return setenv(&mut command_vector),
        "unsetenv"  => return unsetenv(&mut command_vector),
        ""          => {return CommandResult {status: 0, output: None}},
        _           => return launch_bin(&mut command_vector, operator, input)
    }
}

fn handle_and(cmd: &mut Command, cmd_result: &mut CommandResult) {
    if cmd_result.status != 0 {
        return ;
    } else {
        *cmd_result = command_parser(&mut cmd.command, &cmd.command_operator,&cmd_result.output);
    }
}

fn handle_or(cmd: &mut Command, cmd_result: &mut CommandResult) {
    if cmd_result.status == 0 {
        return;
    } else {
        *cmd_result = command_parser(&mut cmd.command, &cmd.command_operator,&cmd_result.output);
    }
}

fn handle_right(cmd: &mut Command, cmd_result: &mut CommandResult) {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = match File::create(&cmd.command[0]) {
        Ok(file) => {file},
        Err(e) => {println!("Could not open file : {}", e); return ;}
    };
    match &cmd_result.output {
        &Some(ref output) => {
            match f.write_all(output.as_bytes()) {
                Ok(_) => {},
                Err(e) => {println!("Error writing to file : {}", e);}
            };
        },
        &None => {}
    }
}

pub fn handle_command(command: &mut String) {

    let mut cmd_result: CommandResult = CommandResult {status: 0, output: None};
    let all_commands: Vec<Command> = get_all_commands(command);
    let mut operator = CommandOperator::NONE;
    let mut idx = 0;

    for mut cmd in all_commands {
        match operator {
            CommandOperator::AND => { handle_and(&mut cmd, &mut cmd_result); },
            CommandOperator::OR  => { handle_or(&mut cmd, &mut cmd_result); },
            CommandOperator::RIGHT => { handle_right(&mut cmd, &mut cmd_result); }
            _ => { cmd_result = command_parser(&mut cmd.command, &cmd.command_operator,&cmd_result.output); }
        }
        idx += 1;
        operator = cmd.command_operator;
    }
}
