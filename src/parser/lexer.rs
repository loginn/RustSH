#[derive(Debug, Clone, PartialEq)]
pub enum TokenOperator {
    Start,
    Cmd,
    And,
    Or,
    Pipe,
    SingleRight,
    SingleLeft,
    Semicolon,
    Eof
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenOperator,
    pub value: Option<String>
}

pub struct Lexer {
    command: String,
    pos: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(command: String) -> Lexer {
        let c = command.clone().chars().next();
        return Lexer {
            command,
            pos: 0,
            current_char: c
        }
    }

    fn peek(&self) -> Option<char> {
        let peek_pos = self.pos + 1;
        if peek_pos > self.command.len() {
            return None;
        }
        return self.command.chars().nth(peek_pos)
    }

    fn command(&mut self) -> Token {
        let mut result: String = String::new();
        while let Some(c) = self.current_char {
            if !";|&><".contains(c) {
                result.push(c);
                self.advance();
            } else if c.is_whitespace() {
                self.skip_whitespace();
            } else {
                break;
            }

        }
        result = result.trim().to_string();
        return Token {
            kind: TokenOperator::Cmd,
            value: Some(result)
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.command.chars().nth(self.pos);
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.command.len() && self.current_char.unwrap().is_whitespace() {
            self.advance();
        }
    }

    fn peek_check(&mut self, c: char) -> bool{
        match self.peek() {
            Some(p) => return p == c,
            None    => return false
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        while self.current_char.is_some() {
            let c = self.current_char.unwrap();
            if c.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if c == ';' {
                self.advance();
                return Token { kind: TokenOperator::Semicolon, value: None }
            } else if c == '|' && self.peek_check(c) {
                self.advance();
                self.advance();
                return Token { kind: TokenOperator::Or, value: None }
            } else if c == '&' && self.peek_check(c) {
                self.advance();
                self.advance();
                return Token { kind: TokenOperator::And, value: None }
            } else if c.is_alphabetic() {
                return self.command();
            } else if c == '>' {
                self.advance();
                return TokenOperator { kind: TokenOperator::SingleRight, value: None}
            } else {
                panic!("Unknown token")
            }
        }
        return Token { kind: TokenOperator::EOF, value: None }
    }

    pub fn set_command(&mut self, command: String) {
        let c = command.clone().chars().next();
        self.command = command;
        self.pos = 0;
        self.current_char = c
    }
}
