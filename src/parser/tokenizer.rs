use std::str::FromStr;

lazy_static! {
    static ref ALLOWED_SPECIAL_CHAR: Vec<char> = vec!['.', '_', '$'];
    static ref ALLOWED_OPERATIONS: Vec<char> = vec!['!', '+', '-', '~', '&', '|'];
}

pub enum Token {
    JumpSymbol(String, u32),
    InstructionEnd,
    ACommandLiteral(u32),
    ACommandSymbol(String),
    Jump(String),
    Destination(String),
    CCommand(String),
}

pub struct Tokenizer {
    raw: Vec<char>,
    current_index: usize,
    line: u32
}

trait CharToken {
    fn is_special(&self) -> bool;
    fn is_operational(&self) -> bool;
}

impl CharToken for char {
    fn is_special(&self) -> bool {
        ALLOWED_SPECIAL_CHAR.contains(self)
    }

    fn is_operational(&self) -> bool {
        ALLOWED_OPERATIONS.contains(self)
    }
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer { raw: vec![], current_index: 0, line: 0 }
    }

    pub fn tokenize(&mut self, source: String) -> Vec<Token> {
        self.raw = source.split("")
            .filter(|s| *s != "" && *s != " ")
            .filter(|s| !s.is_empty())
            .map(|s| char::from_str(s).unwrap())
            .collect();

        let mut tokens: Vec<Token> = vec![];

        while self.has_next() {
            self.scan(&mut tokens);
            if self.has_next() {
                self.advance();
            }
        }

        tokens
    }

    fn scan(&mut self, tokens: &mut Vec<Token>) {
       match self.current() {
            c if c == '@' => {
                let first_char = self.advance();
                let buffer = self.scan_a_command();

                let token = if first_char.is_digit(10) {
                    let literal = u32::from_str(buffer.as_str()).unwrap();
                    Token::ACommandLiteral(literal)
                } else {
                    Token::ACommandSymbol(buffer)
                };

                tokens.push(token);
                self.line +=1;
                self.move_until_new_line();
            },
            c if c.is_alphanumeric() => {
                let dest_buffer = self.scan_c_dest();
                let separator = self.current();
                self.advance();
                let comp_buffer = self.scan_c_comp();

                if separator == ';' {
                    tokens.push(Token::CCommand(dest_buffer));
                    tokens.push(Token::Jump(comp_buffer));
                } else {
                    tokens.push(Token::Destination(dest_buffer));
                    tokens.push(Token::CCommand(comp_buffer));
                };
                self.line +=1;
                self.move_until_new_line();
            },
           ch if ch == '(' => {
               let buffer = self.scan_jump_label();
               tokens.push(Token::JumpSymbol(buffer, self.line));
               self.move_until_new_line();
           },
            ch if ch == '/' && self.peek() == '/' => self.move_until_new_line(),
            _ => {}
        };
    }

    fn scan_a_command(&mut self) -> String {
        let mut buffer = String::new();
        let mut current_char = self.current();

        while current_char.is_alphanumeric() || current_char.is_special() {
            buffer.push(current_char);
            if !self.has_next() {
                break
            }
            current_char = self.advance();
        }

        buffer
    }

    fn scan_c_dest(&mut self) -> String {
        let mut buffer = String::new();
        let mut current_char = self.current();

        while current_char != ';' && current_char != '=' {
            self.throw_if_newline();

            buffer.push(current_char);
            if self.has_next() { current_char = self.advance(); } else { break; }
        }

        buffer
    }

    fn scan_c_comp(&mut self) -> String {
        let mut current_char = self.current();

        let mut buffer = String::new();
        while current_char.is_alphanumeric() || current_char.is_operational() {
            self.throw_if_newline();
            buffer.push(current_char);

            if self.has_next() { current_char = self.advance(); } else { break; }
        }
        buffer
    }

    fn scan_jump_label(&mut self) -> String {
        let mut buffer = String::new();

        while self.has_next() && self.peek() != ')' {
            let current_char = self.advance();
            self.throw_if_newline();

            buffer.push(current_char);
        }

        if !self.has_next() || self.peek() != ')' {
            panic!("Unexpected symbol");
        }

        buffer
    }

    fn throw_if_newline(&mut self) {
        if self.current() == '\n' {
            panic!("Unexpected new line char");
        }
    }

    fn move_until_new_line(&mut self) {
        if self.current() == '\n' {
            return;
        }
        while self.has_next() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn current(&mut self) -> char {
         self.raw[self.current_index]
    }

    fn advance(&mut self) -> char {
        if !self.has_next() {
            panic!("Out of range");
        }

        self.current_index += 1;
        self.raw[self.current_index]
    }

    fn peek(&self) -> char {
        if !self.has_next() {
            panic!("Out of range");
        }

        self.raw[self.current_index + 1]
    }

    fn has_next(&self) -> bool {
        self.current_index < self.raw.len() - 1
        // self.has_next_idx(1)
    }
}

