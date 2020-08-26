use parser::lexer::Token;
use downcast_rs::Downcast;


pub trait ASTNode: Downcast {
    fn type_of(&self) -> &'static str;
}
impl_downcast!(ASTNode);

pub struct BinOp {
    pub left: Box<dyn ASTNode>,
    pub right: Box<dyn ASTNode>,
    pub token: Token
}

pub struct Command {
    pub value: String,
    pub token: Token
}

pub struct Eof {
    pub token: Token
}

impl BinOp {

}

impl ASTNode for Eof {
    fn type_of(&self) -> &'static str {
        return "Eof";
    }
}

impl Command {
    pub fn new(tok: Token) -> Command {
        return Command { token: tok.clone(), value: tok.value.unwrap() }
    }
}

impl ASTNode for BinOp {
    fn type_of(&self) -> &'static str {
        return "BinOp";
    }
}

impl ASTNode for Command {
    fn type_of(&self) -> &'static str {
        return "Command";
    }
}