use parser::lexer::Token;
use parser::lexer::TokenOperator;
use parser::nodes::ASTNode;
use parser::nodes::Command;
use parser::nodes::BinOp;
use parser::lexer::Lexer;
use parser::nodes::Eof;

pub struct Parser {
    pub lexer: Lexer,
    current_token: Token
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            lexer: Lexer::new(String::new()),
            current_token: Token { kind: TokenOperator::Start, value: None }
        }
    }

    pub fn set_command(&mut self, command: String) {
        self.current_token = Token { kind: TokenOperator::Start, value: None };
        self.lexer.set_command(command);
    }

    fn eat(&mut self, kind: TokenOperator) -> Result<Token, &'static str> {
        let token = self.current_token.clone();
        if token.kind == kind {
            self.current_token = self.lexer.get_next_token();
            return Ok(token);
        }
        return Err("Token kind does not match expected type")
    }

    fn factor(&mut self) -> Box<ASTNode> {
        let tok = self.current_token.clone();
        let result: Box<ASTNode> = match tok.kind {
            TokenOperator::Cmd => {
                match self.eat(TokenOperator::Cmd) {
                    Ok(cmd) => {
                        if cmd.value.is_some() {
                            Box::new(Command::new(tok))
                        } else { panic!("Error parsing in factor") }
                    }
                    _   => panic!("Error parsing in factor")
                }
            }
            TokenOperator::Eof => {
                    Box::new(Eof { token: tok })
            }
            _ => {
                panic!("Unknown factor type {:?}", tok.kind)
            }
        };
        return result;
    }

    fn pass(r: Result<Token, &'static str>) {
        match r {
            Ok(_) => {},
            Err(e) => panic!(e)
        }
    }

    pub fn check_token(&mut self) -> bool {
        return self.current_token.kind == TokenOperator::Semicolon
            || self.current_token.kind == TokenOperator::And
            || self.current_token.kind == TokenOperator::Or
            || self.current_token.kind == TokenOperator::SingleRight
            || self.current_token.kind == TokenOperator::DoubleRight
            || self.current_token.kind == TokenOperator::SingleLeft;
    }

    pub fn expr(&mut self) -> Box<ASTNode> {
        if self.current_token.kind == TokenOperator::Start {
            Parser::pass(self.eat(TokenOperator::Start));
        }

        let mut node: Box<ASTNode> = self.factor();

        while self.check_token() {
            let tok = self.current_token.clone();
            if tok.kind == TokenOperator::Semicolon {
                Parser::pass(self.eat(TokenOperator::Semicolon));
            } else if tok.kind == TokenOperator::And {
                Parser::pass(self.eat(TokenOperator::And));
            } else if tok.kind == TokenOperator::Or {
                Parser::pass(self.eat(TokenOperator::Or));
            } else if tok.kind == TokenOperator::SingleRight {
                Parser::pass(self.eat(TokenOperator::SingleRight));
            } else if tok.kind == TokenOperator::DoubleRight{
                Parser::pass(self.eat(TokenOperator::DoubleRight));
            } else if tok.kind == TokenOperator::SingleLeft {
                Parser::pass(self.eat(TokenOperator::SingleLeft));
            }

            node = Box::new(BinOp {
                left: node,
                right: self.factor(),
                token: tok
            })
        }
        return node
    }
}
