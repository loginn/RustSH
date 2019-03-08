use command_handler::{CommandResult};
use std::process::{Stdio};
use launch_bin::launch_bin;
use cd::cd;
use env::env;
use env::setenv;
use env::unsetenv;
use parser::parser::Parser;
use parser::nodes::BinOp;
use parser::nodes::Command;
use parser::lexer::TokenOperator;
use parser::nodes::ASTNode;
use std::cmp::max;
use std::fs::File;

pub trait NodeVisitor {
    fn visit(&self, node: Box<ASTNode>, stdin: Option<Stdio>, stdout: Option<Stdio>) -> CommandResult;
}


pub struct Interpreter {
    pub parser: Parser
}

impl NodeVisitor for Interpreter {
    fn visit(&self, node: Box<ASTNode>, stdin: Option<Stdio>, stdout: Option<Stdio>) -> CommandResult {
        if node.type_of() == "BinOp" {
            return self.visit_binop(node.downcast::<BinOp>().ok().unwrap())
        } else if node.type_of() == "Eof" {
            return CommandResult { child: None, status: 0 }
        } else {
            return self.visit_command(node.downcast::<Command>().ok().unwrap(), stdin, stdout);
        }
    }
}

impl Interpreter {
    fn set_command(&mut self, command: String) {
        self.parser.set_command(command);
    }

    fn visit_command(&self, node: Box<Command>, stdin: Option<Stdio>, stdout: Option<Stdio>) -> CommandResult {
        let mut cmd: Vec<String> = node.value.clone().split(' ').map(|x: &str| x.to_string()).collect();

        let result = match cmd.as_str() {
            "cd"        => cd(&cmd.split(' ').map(|x: &str| x.to_string()).collect()),
            "env"       => env(),
            "setenv"    => setenv(&mut cmd.split(' ').map(|x: &str| x.to_string()).collect()),
            "unsetenv"  => unsetenv(&mut cmd.split(' ').map(|x: &str| x.to_string()).collect()),
            _           => launch_bin(&mut cmd.split(' ').map(|x: &str| x.to_string()).collect())
        };
        return result
    }

    fn visit_binop(&self, node: Box<BinOp>) -> CommandResult {
        let n = *node;
        match n.token.kind {
            TokenOperator::Semicolon => {
                self.visit(n.left, None, None);
                return self.visit(n.right, None, None);
            }
            TokenOperator::And => {
                let cr1 = self.visit(n.left, None, None);
                if cr1.status == 0 {
                    let cr2 = self.visit(n.right, None, None);
                    CommandResult { child: None, status: max(cr1.status, cr2.status) }
                } else {
                    CommandResult { child: None, status: cr1.status }
                }
            }
            TokenOperator::Or => {
                let cr1 = self.visit(n.left, None, None);
                if cr1.status != 0 {
                    let cr2 = self.visit(n.right, None, None);
                    CommandResult { child: None, status: max(cr1.status, cr2.status) }
                } else {
                    CommandResult { child: None, status: cr1.status }
                }
            } TokenOperator::SingleRight => {
                let path = n.right.downcast::<Command>().ok().unwrap().value;
                match  File::create(&path) {
                    Ok(f) => {
                        return self.visit(n.left, None, Some(f.into()));
                    },
                    Err(_) => {
                        println!("RustSH : file not found : {}", path);
                    },
                };
                CommandResult { child: None, status: 1 }
            } TokenOperator::SingleLeft => {
                let path = n.right.downcast::<Command>().ok().unwrap().value;

                match  File::open(&path) {
                    Ok(f) => {
                        return self.visit(n.left, Some(f.into()), None);
                    },
                    Err(_) => {
                        println!("RustSH: {} : file not found", path);
                    },
                };
                CommandResult { child: None, status: 1 }
            }
            _ => {unimplemented!()}
        }
    }

    pub fn interpret(&mut self, command: String) {
        self.set_command(command);
        let node = self.parser.expr();
        self.visit(node, None, None);
    }
}
