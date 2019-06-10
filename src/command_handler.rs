extern crate regex;
use std::process::{Child, Output};

#[derive(Debug)]
pub struct CommandResult {
    pub child: Option<Child>,
    pub output: Option<Output>,
    pub status: Option<i32>
}