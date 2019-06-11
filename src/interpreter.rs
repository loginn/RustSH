use cd::cd;
use command_handler::CommandResult;
use env::env;
use env::setenv;
use env::unsetenv;
use launch_bin::launch_bin;
use parser::lexer::TokenOperator;
use parser::nodes::ASTNode;
use parser::nodes::BinOp;
use parser::nodes::Command;
use parser::parser::Parser;
use std::fs::{File, OpenOptions};
use std::process::{Child, Stdio, Output};
use std::io::Write;

pub trait NodeVisitor {
    fn visit(&mut self, node: &Box<dyn ASTNode>, stdin: Option<Stdio>, stdout: Option<Stdio>) -> CommandResult;
}


pub struct Interpreter {
    pub parser: Parser
}

impl NodeVisitor for Interpreter {
    fn visit(&mut self, node: &Box<dyn ASTNode>, stdin: Option<Stdio>, stdout: Option<Stdio>) -> CommandResult {
        if node.type_of() == "BinOp" {
            self.visit_binop(&node.downcast_ref::<BinOp>().unwrap())
        } else {
            self.visit_command(&node.downcast_ref::<Command>().unwrap(), stdin, stdout)
        }
    }
}

impl Interpreter {
    fn set_command(&mut self, command: String) {
        self.parser.set_command(command);
    }

    fn visit_command(&mut self, node: &Command, stdin: Option<Stdio>, stdout: Option<Stdio>) -> CommandResult {
        let mut cmd: Vec<String> = node.value.clone().split(' ').map(|x: &str| x.to_string()).collect();

        let result = match cmd[0].as_str() {
            "cd"        => cd(&mut cmd),
            "env"       => env(),
            "setenv"    => setenv(&mut cmd),
            "unsetenv"  => unsetenv(&mut cmd),
            _           => CommandResult {child : launch_bin(&mut cmd, stdin, stdout), output: None, status: None}
        };
        return result
    }

    fn get_child(&mut self, result: CommandResult) -> Option<Child> {
        match result.child {
            Some(ch) => {
                Some(ch)
            },
            None => {
                None
            }
        }
    }

    fn wait_for_child(&mut self, result: CommandResult) -> Option<Output> {
        match self.get_child(result) {
            Some(ch) => {
                match ch.wait_with_output() {
                    Ok(output) => {
                        Some(output)
                    },
                    Err(_) => None
                }
            },
            None => {
                None
            }
        }
    }

    fn binop_pipe(&mut self, n: &BinOp) -> CommandResult {
        let pipe1 = self.visit(&n.left, None, Some(Stdio::piped()));
        let cr = match pipe1.output {
            None => {
                let out1 = self.wait_for_child(pipe1);
                let mut pipe2 = self.visit(&n.right, Some(Stdio::piped()), Some(Stdio::piped()));
                pipe2.child.as_mut().unwrap().stdin.as_mut().unwrap().write_all(out1.unwrap().stdout.as_slice()).ok();

                let output = self.wait_for_child(pipe2);

                CommandResult {
                    child: None,
                    output,
                    status: None
                }
            },
            Some(out1) => {
                let mut pipe = self.visit(&n.right, Some(Stdio::piped()), Some(Stdio::piped()));
                pipe.child.as_mut().unwrap().stdin.as_mut().unwrap().write_all(out1.stdout.as_slice()).ok();

                let output = self.wait_for_child(pipe);

                CommandResult {
                    child: None,
                    output,
                    status: None
                }
            }
        };
        return cr;
    }


    fn binop_semi(&mut self, n: &BinOp) -> CommandResult {
        let r = self.visit(&n.left, None, None);
        self.wait_for_child(r);
        let r = self.visit(&n.right, None, None);
        CommandResult{child: None, output: Some(self.wait_for_child(r).unwrap()), status: None}
    }

    fn binop_and(&mut self, n: &BinOp) -> CommandResult {
        let r = self.visit(&n.left, None, None);
        let output_1 = self.wait_for_child(r).unwrap();
        let status_1 = output_1.status.success();


        if status_1 {
            let r = self.visit(&n.right, None, None);
            let output_2 = self.wait_for_child(r).unwrap();
            let status_2 = output_2.status.success();
            if !status_2 {
                CommandResult { child: None, output: Some(output_2), status: None }
            } else {
                CommandResult { child: None, output: Some(output_1), status: None }
            }
        } else {
            CommandResult { child: None, output: Some(output_1), status: None }
        }
    }

    fn binop_or(&mut self, n: &BinOp) -> CommandResult {
        let r = self.visit(&n.left, None, None);
        let output_1 = self.wait_for_child(r).unwrap();
        let status_1 = output_1.status.code().unwrap();

        if status_1 != 0 {
            let r = self.visit(&n.right, None, None);
            let output_2 = self.wait_for_child(r).unwrap();
            return CommandResult { child: None, output: Some(output_2), status: None }
        }
        return CommandResult { child: None, output: Some(output_1), status: None }
    }

    fn binop_single_right(&mut self, n: &BinOp) -> CommandResult {
        let path = &n.right.downcast_ref::<Command>().unwrap().value;
        match  File::create(&path) {
            Ok(mut f) => {
                let r = self.visit(&n.left, None, None);
                match r.output {
                    Some(out) => {
                        f.write_all(out.stdout.as_ref()).ok();
                    },
                    None => {
                        let output = self.wait_for_child(r);
                        match output {
                            Some(out) => {
                                f.write_all(out.stdout.as_ref()).ok();
                            },
                            None => {}
                        }
                    }
                }

                CommandResult { child: None, output: None, status: None }
            },
            Err(_) => {
                println!("RustSH : File error : {}", path);
                CommandResult { child: None, output: None, status: Some(1) }
            },
        };
        CommandResult { child: None, output: None, status: None }
    }

    fn binop_double_right(&mut self, n: &BinOp) -> CommandResult {
        let path = &n.right.downcast_ref::<Command>().unwrap().value;
        match  OpenOptions::new().append(true).create(true).open(&path) {
            Ok(mut f) => {
                let r = self.visit(&n.left, None, None);
                match r.output {
                    Some(out) => {
                        f.write_all(out.stdout.as_ref()).ok();
                    },
                    None => {
                        let output = self.wait_for_child(r);
                        match output {
                            Some(out) => {
                                f.write_all(out.stdout.as_ref()).ok();
                            },
                            None => {}
                        }
                    }
                }

                CommandResult { child: None, output: None, status: None }
            },
            Err(_) => {
                println!("RustSH : Could not open file : {}", path);
                CommandResult { child: None, output: None, status: Some(1) }
            },
        };
        CommandResult { child: None, output: None, status: None}
    }

    fn binop_single_left(&mut self, n: &BinOp) -> CommandResult {
        let path = &n.right.downcast_ref::<Command>().unwrap().value;

        match File::open(&path) {
            Ok(f) => {
                let r = self.visit(&n.left, Some(f.into()), None);
                let output = self.wait_for_child(r);
                CommandResult { child: None, output, status: None }
            },
            Err(_) => {
                println!("RustSH: {} : file not found", path);
                CommandResult { child: None, output: None, status: Some(1) }
            },
        };
        CommandResult { child: None, output: None, status: None }
    }

    fn visit_binop(&mut self, node: &BinOp) -> CommandResult {
        let n = *&node;
        match n.token.kind {
            TokenOperator::Semicolon => { return self.binop_semi(node) }
            TokenOperator::And => { return self.binop_and(&node) }
            TokenOperator::Or => { return self.binop_or(&node) }
            TokenOperator::SingleRight => { return self.binop_single_right(&node) }
            TokenOperator::DoubleRight => { return self.binop_double_right(&node) }
            TokenOperator::SingleLeft => { return self.binop_single_left(&node) }
            TokenOperator::Pipe => { self.binop_pipe(&node) }
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
        let mut result = self.visit(&node, None, None);

        match &mut result.child {
            Some(_) => {
                match self.wait_for_child(result) {
                    Some(out) => {
                        if !out.stdout.is_empty() {
                            println!("{}", std::str::from_utf8(out.stdout.as_slice()).ok().unwrap());
                        }
                    },
                    None => {}
                }
            },
            None => {
                match result.output {
                    Some(out) => {
                        if !out.stdout.is_empty() {
                            print!("{}", std::str::from_utf8(out.stdout.as_slice()).ok().unwrap());
                        }
                    },
                    None => {}
                }
            }
        }
    }
}
