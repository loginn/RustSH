use command_handler::{CommandResult};
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

pub trait NodeVisitor {
    fn visit(&self, node: Box<ASTNode>) -> CommandResult;
}


pub struct Interpreter {
    pub parser: Parser
}

impl NodeVisitor for Interpreter {
    fn visit(&self, node: Box<ASTNode>) -> CommandResult {
        if node.type_of() == "BinOp" {
            return self.visit_binop(node.downcast::<BinOp>().ok().unwrap())
        } else if node.type_of() == "Eof" {
            return CommandResult { child: None, status: 0 }
        } else {
            return self.visit_command(node.downcast::<Command>().ok().unwrap())
        }
    }
}

impl Interpreter {
    fn set_command(&mut self, command: String) {
        self.parser.set_command(command);
    }

    fn visit_command(&self, node: Box<Command>) -> CommandResult {
        let cmd = node.value.clone();

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
                self.visit(n.left);
                return self.visit(n.right);
            }
            TokenOperator::And => {
                let cr1 = self.visit(n.left);
                if cr1.status == 0 {
                    let cr2 = self.visit(n.right);
                    CommandResult { child: None, status: max(cr1.status, cr2.status) }
                } else {
                    CommandResult { child: None, status: cr1.status }
                }
            }
            TokenOperator::Or => {
                let cr1 = self.visit(n.left);
                if cr1.status != 0 {
                    let cr2 = self.visit(n.right);
                    CommandResult { child: None, status: max(cr1.status, cr2.status) }
                } else {
                    CommandResult { child: None, status: cr1.status }
                }
            }
            _ => {unimplemented!()}
        }
    }

    pub fn interpret(&mut self, command: String) {
        self.set_command(command);
        let node = self.parser.expr();
        self.visit(node);
    }
}
