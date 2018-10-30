extern crate regex;
use std::process::Command;

#[derive(Debug)]
pub struct CommandResult {
    pub child: Option<Command>,
    pub status: i32,
}