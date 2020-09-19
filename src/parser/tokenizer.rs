
use std::str::FromStr;

lazy_static! {
    static ref ALLOWED_NAME_SYMBOLS: Vec<char> = vec!['.', '_', '$'];
    static ref ALLOWED_OPERATIONS: Vec<char> = vec!['.', '_', '+',];
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
            self.process_next(&mut tokens);


            if self.has_next() {
                self.advance();
            }
        }

        tokens
    }

    // fn process_a_command(&mut self) -> Option<Token> {
    //     let first_char = self.char();
    //     let mut buffer = String::new();
    //
    //     while self.char().is_alphabetic() || self.char().is_digit(10) || ALLOWED_NAME_SYMBOLS.contains(&self.char()) {
    //         buffer.push(self.char());
    //         self.advance();
    //     }
    //
    //     let token = if first_char.is_digit(10) {
    //         let literal = u32::from_str(buffer.as_str()).unwrap();
    //         Token::ACommandLiteral(literal)
    //     } else {
    //         Token::ACommandSymbol(buffer)
    //     };
    //     Some(token)
    // }

    fn process_next(&mut self, tokens: &mut Vec<Token>) {
       match self.char() {
            // Handling ACommand
            c if c == '@' => {
                let mut peek = self.advance();
                let first_char = peek;
                let mut buffer = String::new();

                while self.is_allowed_variable_char(peek) {
                    buffer.push(peek);
                    if !self.has_next() {
                        break
                    }
                    peek = self.advance();
                }

                let token = if first_char.is_digit(10) {
                    let literal = u32::from_str(buffer.as_str()).unwrap();
                    Token::ACommandLiteral(literal)
                } else {
                    Token::ACommandSymbol(buffer)
                };

                tokens.push(token);
                self.line +=1;
                self.move_to_next_line();
            },
            c if self.is_allowed_variable_char(c) => {
                let mut dest_buffer = String::new();
                let mut ch = self.char();

                while ch != ';' && ch != '=' {
                    if ch == '\n' {
                        panic!("Unexpected new line char");
                    }

                    dest_buffer.push(ch);

                    if self.has_next() {
                        ch = self.advance();
                    } else {
                        break;
                    }
                }

                ch = self.char();
                let is_jump_cmd = ch == ';';
                ch = self.advance();

                let mut command_buffer = String::new();
                while self.is_allowed_variable_char(ch) || vec!['!', '+', '-', '~', '&', '|'].contains(&ch) {
                    if ch == '\n' {
                        panic!("Unexpected new line char");
                    }

                    command_buffer.push(ch);

                    if self.has_next() {
                        ch = self.advance();
                    } else {
                        break;
                    }
                }

                if is_jump_cmd {
                    tokens.push(Token::CCommand(dest_buffer));
                    tokens.push(Token::Jump(command_buffer));
                } else {
                    tokens.push(Token::Destination(dest_buffer));
                    tokens.push(Token::CCommand(command_buffer));
                };
                self.line +=1;
                self.move_to_next_line();
            },
           ch if ch == '(' => {
               let mut symbol = String::new();

               while self.has_next() && self.peek() != ')' {
                   let char = self.advance();
                   if char == '\n' {
                       panic!("Unexpected new line char");
                   }

                   symbol.push(char);
               }

               if !self.has_next() || self.peek() != ')' {
                   panic!("GotoSymbol parsing exception");
               }

               tokens.push(Token::JumpSymbol(symbol, self.line));
               self.move_to_next_line();
           },
            ch if ch == '/' && self.peek() == '/' => {
                self.move_to_next_line();
            },
            _ => {}
        };
    }

    fn process_comment(&mut self) -> Option<Token> {
        let comment_line = if !self.has_prev() || self.prev() == '\n' {
            true
        } else {
            false
        };

        self.advance();

        while self.has_next() && self.char() != '\n' {
            self.advance();
        }

        if !comment_line {
            return Some(Token::InstructionEnd);
        }

        Option::None
    }

    fn move_to_next_line(&mut self) {
        if self.char() == '\n' {
            return;
        }
        while self.has_next() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn is_allowed_variable_char(&self, ch: char) -> bool {
        return ch.is_alphabetic() || ch.is_digit(10) || ALLOWED_NAME_SYMBOLS.contains(&ch);
    }

    fn char(&mut self) -> char {
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

    // fn peek_idx(&self, index: i32) -> char {
    //     if self.current_index >= self.raw.len() - index {
    //         panic!("Out of range");
    //     }
    //
    //     self.raw[self.current_index + index]
    // }

    fn has_next(&self) -> bool {
        self.current_index < self.raw.len() - 1
        // self.has_next_idx(1)
    }

    // fn has_next_idx(&self, index: u16) -> bool {
    //     self.current_index < self.raw.len() - index
    // }

    fn prev(&self) -> char {
        if !self.has_prev() {
            panic!("Out of range");
        }

        self.raw[self.current_index - 1]
    }

    fn has_prev(&self) -> bool {
        self.current_index > 0
    }
}

