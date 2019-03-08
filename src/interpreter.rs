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
use std::fs::{File, OpenOptions};

pub trait NodeVisitor {
    fn visit(&self, node: &Box<ASTNode>, stdin: Option<Stdio>, stdout: Option<Stdio>) -> CommandResult;
}


pub struct Interpreter {
    pub parser: Parser
}

impl NodeVisitor for Interpreter {
    fn visit(&self, node: &Box<ASTNode>, stdin: Option<Stdio>, stdout: Option<Stdio>) -> CommandResult {
        if node.type_of() == "BinOp" {
            return self.visit_binop(&node.downcast_ref::<BinOp>().unwrap())
        } else if node.type_of() == "Eof" {
            return CommandResult { child: None, status: 0 }
        } else {
            return self.visit_command(&node.downcast_ref::<Command>().unwrap(), stdin, stdout);
        }
    }
}

impl Interpreter {
    fn set_command(&mut self, command: String) {
        self.parser.set_command(command);
    }

    fn visit_command(&self, node: &Command, stdin: Option<Stdio>, stdout: Option<Stdio>) -> CommandResult {
        let mut cmd: Vec<String> = node.value.clone().split(' ').map(|x: &str| x.to_string()).collect();

        let result = match cmd[0].as_str() {
            "cd"        => cd(&mut cmd),
            "env"       => env(),
            "setenv"    => setenv(&mut cmd),
            "unsetenv"  => unsetenv(&mut cmd),
            _           => launch_bin(&mut cmd, stdin, stdout)
        };
        return result
    }

    fn binop_semi(&self, n: &BinOp) -> CommandResult {
        self.visit(&n.left, None, None);
        return self.visit(&n.right, None, None);

    }

    fn binop_and(&self, n: &BinOp) -> CommandResult {
        let cr1 = self.visit(&n.left, None, None);
        if cr1.status == 0 {
            let cr2 = self.visit(&n.right, None, None);
            CommandResult { child: None, status: max(cr1.status, cr2.status) }
        } else {
            CommandResult { child: None, status: cr1.status }
        }
    }

    fn binop_or(&self, n: &BinOp) -> CommandResult {
        let cr1 = self.visit(&n.left, None, None);
        if cr1.status != 0 {
            let cr2 = self.visit(&n.right, None, None);
            CommandResult { child: None, status: max(cr1.status, cr2.status) }
        } else {
            CommandResult { child: None, status: cr1.status }
        }
    }

    fn binop_single_right(&self, n: &BinOp) -> CommandResult {
        let path = &n.right.downcast_ref::<Command>().unwrap().value;
        match  File::create(&path) {
            Ok(f) => {
                return self.visit(&n.left, None, Some(f.into()));
            },
            Err(_) => {
                println!("RustSH : file not found : {}", path);
            },
        };
        CommandResult { child: None, status: 1 }
    }

    fn binop_double_right(&self, n: &BinOp) -> CommandResult {
        let path = &n.right.downcast_ref::<Command>().unwrap().value;
        match  OpenOptions::new().append(true).create(true).open(&path) {
            Ok(f) => {
                return self.visit(&n.left, None, Some(f.into()));
            },
            Err(_) => {
                println!("RustSH : Could not open file : {}", path);
            },
        };
        CommandResult { child: None, status: 1 }
    }

    fn binop_single_left(&self, n: &BinOp) -> CommandResult {
        let path = &n.right.downcast_ref::<Command>().unwrap().value;

        match File::open(&path) {
            Ok(f) => {
                return self.visit(&n.left, Some(f.into()), None);
            },
            Err(_) => {
                println!("RustSH: {} : file not found", path);
            },
        };
        CommandResult { child: None, status: 1 }
    }

    fn visit_binop(&self, node: &BinOp) -> CommandResult {
        let n = *&node;
        match n.token.kind {
            TokenOperator::Semicolon => { self.binop_semi(node) }
            TokenOperator::And => { self.binop_and(&node) }
            TokenOperator::Or => { self.binop_or(&node) }
            TokenOperator::SingleRight => { self.binop_single_right(&node) }
            TokenOperator::DoubleRight => { self.binop_double_right(&node) }
            TokenOperator::SingleLeft => { self.binop_single_left(&node) }
            _ => {unimplemented!()}
        }
    }

    pub fn interpret(&mut self, command: String) {
        self.set_command(command);
        let node = self.parser.expr();
        self.visit(&node, None, None);
    }
}
