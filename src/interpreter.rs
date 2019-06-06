use command_handler::{CommandResult};
use std::process::{Stdio, Child};
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
    fn visit(&mut self, node: &Box<dyn ASTNode>, stdin: Option<Stdio>, stdout: Option<Stdio>);
}


pub struct Interpreter {
    pub parser: Parser,
    pub current_result: Option<CommandResult>
}

impl NodeVisitor for Interpreter {
    fn visit(&mut self, node: &Box<dyn ASTNode>, stdin: Option<Stdio>, stdout: Option<Stdio>) {
        if node.type_of() == "BinOp" {
            self.visit_binop(&node.downcast_ref::<BinOp>().unwrap());
        } else {
            self.visit_command(&node.downcast_ref::<Command>().unwrap(), stdin, stdout);
        }
    }
}

impl Interpreter {
    fn set_command(&mut self, command: String) {
        self.parser.set_command(command);
    }

    fn visit_command(&mut self, node: &Command, stdin: Option<Stdio>, stdout: Option<Stdio>) {
        let mut cmd: Vec<String> = node.value.clone().split(' ').map(|x: &str| x.to_string()).collect();

        let result = match cmd[0].as_str() {
            "cd"        => cd(&mut cmd),
            "env"       => env(),
            "setenv"    => setenv(&mut cmd),
            "unsetenv"  => unsetenv(&mut cmd),
            _           => CommandResult {child : launch_bin(&mut cmd, stdin, stdout), status: 0}
        };
        self.current_result = Some(result)
    }

    fn get_child(&mut self) -> Option<&mut Child> {
        match &mut self.current_result {
            Some(cr) => {
                match &mut cr.child {
                    Some(ch) => {
                        Some(ch)
                    },
                    None => {
                        None
                    }
                }
            },
            None => {
                None
            }
        }
    }

    fn wait_for_child(&mut self) -> i32 {
        match self.get_child() {
            Some(ch) => {
                match ch.wait() {
                    Ok(status) => {
                        return status.code().unwrap()
                    },
                    Err(_) => 1
                }
            },
            None => {
                1
            }
        }
    }

//    fn binop_pipe(&mut self, n: &BinOp) {
//        self.visit(&n.left, None, Some(Stdio::piped()));
//        let out = child.wait_with_output();
//        self.visit(&n.right, Some(Stdio::piped()), None);
//    }


    fn binop_semi(&mut self, n: &BinOp) {
        self.visit(&n.left, None, None);
        self.wait_for_child();
        self.visit(&n.right, None, None);
        self.wait_for_child();
    }

    fn binop_and(&mut self, n: &BinOp) {
        self.visit(&n.left, None, None);
        let status_1 = self.wait_for_child();

        if status_1 == 0 {
            self.visit(&n.right, None, None);
            let status_2 = self.wait_for_child();
            self.current_result = Some(CommandResult { child: None, status: max(status_1, status_2) });
        } else {
            self.current_result = Some(CommandResult { child: None, status: status_1 });
        }
    }

    fn binop_or(&mut self, n: &BinOp) {
        self.visit(&n.left, None, None);
        let status_1 = self.wait_for_child();
        if status_1 != 0 {
            self.visit(&n.right, None, None);
            self.wait_for_child();
        }
    }

    fn binop_single_right(&mut self, n: &BinOp) {
        let path = &n.right.downcast_ref::<Command>().unwrap().value;
        match  File::create(&path) {
            Ok(f) => {
                self.visit(&n.left, None, Some(f.into()));
                self.wait_for_child();
            },
            Err(_) => {
                println!("RustSH : File error : {}", path);
            },
        };
    }

    fn binop_double_right(&mut self, n: &BinOp) {
        let path = &n.right.downcast_ref::<Command>().unwrap().value;
        match  OpenOptions::new().append(true).create(true).open(&path) {
            Ok(f) => {
                self.visit(&n.left, None, Some(f.into()));
                self.wait_for_child();
            },
            Err(_) => {
                println!("RustSH : Could not open file : {}", path);
            },
        };
    }

    fn binop_single_left(&mut self, n: &BinOp) {
        let path = &n.right.downcast_ref::<Command>().unwrap().value;

        match File::open(&path) {
            Ok(f) => {
                self.visit(&n.left, Some(f.into()), None);
                self.wait_for_child();
            },
            Err(_) => {
                println!("RustSH: {} : file not found", path);
            },
        };
    }

    fn visit_binop(&mut self, node: &BinOp) {
        let n = *&node;
        match n.token.kind {
            TokenOperator::Semicolon => { self.binop_semi(node) }
            TokenOperator::And => { self.binop_and(&node) }
            TokenOperator::Or => { self.binop_or(&node) }
            TokenOperator::SingleRight => { self.binop_single_right(&node) }
            TokenOperator::DoubleRight => { self.binop_double_right(&node) }
            TokenOperator::SingleLeft => { self.binop_single_left(&node) }
//            TokenOperator::Pipe => { self.binop_pipe(&node) }
            _ => {unimplemented!()}
        }
    }

//    fn dbug_tree(&self, node: &Box<dyn ASTNode>) {
//        if node.type_of() == "BinOp" {
//            let n: &BinOp = &node.downcast_ref::<BinOp>().unwrap();
//            println!("{:?}", n.token);
//            self.dbug_tree(&n.left);
//            self.dbug_tree(&n.right);
//        } else if node.type_of() == "Eof" {
//            println!("EOF")
//        } else {
//            let n: &Command = &node.downcast_ref::<Command>().unwrap();
//            println!("command {:?}", n.value);
//        }
//    }

    pub fn interpret(&mut self, command: String) {
        self.set_command(command);
        let node = self.parser.expr();
        self.visit(&node, None, None);

        match &mut self.current_result {
            Some(cr) => {
                match &mut cr.child {
                    Some(_) => {
                        self.wait_for_child();
                    },
                    None => {}
                }
            },
            None => {}
        }
    }
}
