extern crate regex;
use std::process::Child;

#[derive(Debug)]
pub struct CommandResult {
    pub child: Option<Child>,
    pub status: i32,
}