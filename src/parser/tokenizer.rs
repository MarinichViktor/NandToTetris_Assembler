
use std::str::FromStr;

lazy_static! {
    static ref ALLOWED_NAME_SYMBOLS: Vec<char> = vec!['.', '_', '$'];
    static ref ALLOWED_OPERATIONS: Vec<char> = vec!['.', '_', '+'];
}

pub enum OperationType {
    Add,
    Subtract,
    Assign
}

pub enum JumpType {
    Null,
    JNG,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

pub enum CommandType {
    Zero,
    One,
    MinusOne,
    D,
    A,
    NotD,
    NotA,
    MinusD,
    MinusA,
    DPlusOne,
    APlusOne,
    DMinusOne,
    AMinusOne,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
    M,
    NotM,
    MinusM,
    MPlusOne,
    MMinusOne,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM,
}

pub enum Token {
    Literal(i32),
    Symbol(String),
    Variable(String),
    Operation(String),
    JumpSymbol(String, u32),
    AAssignment,
    Semicolon,
    ARegister,
    DRegister,
    Memory,
    InstructionEnd,

    // NEW
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
        self.raw = source.split("").filter(|s| *s != "" || *s != " ").into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| char::from_str(s).unwrap()).collect();
        let mut tokens: Vec<Token> = vec![];

        while self.has_next() {
            self.process_next(&mut tokens);
            if self.has_next() {
                self.advance();
            }
        }

        tokens
    }

    fn process_a_command(&mut self) -> Option<Token> {
        let first_char = self.char();
        let mut buffer = String::new();

        while self.char().is_alphabetic() || self.char().is_digit(10) || ALLOWED_NAME_SYMBOLS.contains(&self.char()) {
            buffer.push(self.char());
            self.advance();
        }

        let token = if first_char.is_digit(10) {
            let literal = u32::from_str(buffer.as_str()).unwrap();
            Token::ACommandLiteral(literal)
        } else {
            Token::ACommandSymbol(buffer)
        };
        Some(token)
    }

    fn process_next(&mut self, tokens: &mut Vec<Token>) {
       match self.char() {
            // Handling ACommand
            c if c == '@' => {
                self.advance();
                if let Some(x) = self.process_a_command() {
                    tokens.push(x)
                }
                self.line +=1;
            },
            c if c.is_alphabetic() || c.is_digit(10) || ALLOWED_NAME_SYMBOLS.contains(&c) => {
                let mut dest_buffer = String::new();

                while self.has_next() && self.char() != ';' && self.char() != '=' {
                    let ch = self.char();
                    if ch == ' ' {
                       self.advance();
                    }

                    dest_buffer.push(ch);
                    self.advance();
                }

                if !self.has_next() {
                    return;
                }


                let is_jump_cmd = self.char() == ';';
                self.advance();
                let mut command_buffer = String::new();
                while self.char().is_alphabetic() || self.char().is_digit(10) || vec!['!', '+', '-', '~'].contains(&self.char()) {
                    if self.char() == ' ' {
                        self.advance();
                    }

                    command_buffer.push(self.char());
                    self.advance();
                }

                if is_jump_cmd {
                    tokens.push(Token::CCommand(dest_buffer));
                    tokens.push(Token::Jump(command_buffer));
                } else {
                    tokens.push(Token::Destination(dest_buffer));
                    tokens.push(Token::CCommand(command_buffer));
                };
                self.line +=1;
            },
           ch if ch == '(' => {
               let mut symbol = String::new();

               while self.has_next() && self.peek() != ')' {
                   self.advance();
                   symbol.push(self.char());
               }

               if !self.has_next() || self.peek() != ')' {
                   panic!("GotoSymbol parsing exception");
               }
               tokens.push(Token::JumpSymbol(symbol, self.line));
           },
            ch if ch == '/' && self.peek() == '/' => {
                if let Some(x) = self.process_comment() {
                    tokens.push(x);
                }
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

    fn char(&mut self) -> char {
        if !self.has_next() {
            panic!("Out of range");
        }

        self.raw[self.current_index]
    }

    fn advance(&mut self) {
        if !self.has_next() {
            panic!("Out of range");
        }

        self.current_index += 1;
    }

    // fn release(&mut self) {
    //     if !self.has_prev() {
    //         panic!("Out of range");
    //     }
    //
    //     self.current_index -= 1;
    // }

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

