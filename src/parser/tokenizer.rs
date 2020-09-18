
use std::error::Error;
use std::borrow::Borrow;
use std::thread::current;
use std::str::FromStr;
use crate::parser::tokenizer::Token::Operation;
use crate::parser::tokenizer::JumpType::{JNG, JGT, JEQ, JGE, JLT, JNE, JLE, JMP, Null};
use std::hash::Hash;

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
    JumpSymbol(String),
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
// let jump_token = match command_buffer.as_str() {
//     "JNG" => JNG,
//     "JGT" => JGT,
//     "JEQ" => JEQ,
//     "JGE" => JGE,
//     "JLT" => JLT,
//     "JNE" => JNE,
//     "JLE" => JLE,
//     "JMP" => JMP,
//     _ => panic!("Parse error")
// };


// const ALLOWED_NAME_SYMBOLS: Vec<char> = vec!['.', '_', '$'];
// const ALLOWED_OPERATIONS: Vec<char> = vec!['.', '_', '+'];

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer { raw: vec![], current_index: 0, line: 1 }
    }

    pub fn tokenize(&mut self, source: String) -> Vec<Token> {
        self.raw = source.split("").into_iter()
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

                // let first_char = self.char();
                // let mut buffer = String::new();
                //
                // while self.char().is_alphabetic() || self.char().is_digit(10) || ALLOWED_NAME_SYMBOLS.contains(&self.char()) {
                //     buffer.push(self.char());
                //     self.advance();
                // }
                //
                // let token = if first_char.is_digit(10) {
                //     let literal = u32::from_str(buffer.as_str()).unwrap();
                //     Token::ACommandLiteral(literal)
                // } else {
                //     Token::ACommandSymbol(buffer)
                // };
            },
            c if c.is_alphabetic() => {
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

                tokens.push(Token::Destination(dest_buffer));

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

                let result = if is_jump_cmd {
                    Token::Jump(command_buffer)
                } else {
                    Token::CCommand(command_buffer)
                };
                tokens.push(result);
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
            self.line += 1;
            return Some(Token::InstructionEnd);
        }

        Option::None
    }

    // fn parse_raw(&mut self) -> Option<Token> {
    //     match self.char() {
    //         ch if ch == '@' => Option::Some(Token::AAssignment),
    //         // TODO revrite to handle as single Jump
    //         ch if ch == ';' => Option::Some(Token::Semicolon),
    //         ch if ch == '(' => {
    //             let mut symbol = String::new();
    //
    //             while self.has_next() && self.peek() != ')' {
    //                 self.advance();
    //                 symbol.push(self.char());
    //             }
    //
    //             if !self.has_next() || !self.peek() != ')' {
    //                 panic!("GotoSymbol parsing exception");
    //             }
    //
    //             Some(Token::JumpSymbol(symbol))
    //         },
    //         ch if (ch == 'A' || ch == 'D' || ch == 'M') && self.peek() == '=' => {
    //
    //         },
    //
    //
    //         ch if ch == '-' || ch == '+' || ch == '=' => {
    //             let op_type = match ch {
    //                 '-' => OperationType::Subtract,
    //                 '+' => OperationType::Add,
    //                 '=' => OperationType::Assign,
    //                 _ => panic!("Invalid operation")
    //             };
    //
    //             Some(Token::Operation(op_type))
    //         },
    //         // TODO: handle A=D+1 as `A=` -> Token::Destination , `D+1` -> Token::Operation
    //         ch if ch >= 'A' && ch <= 'Z' => {
    //             // Handle dest
    //             if self.has_next_idx(3) {
    //                 let mut cmd = String::new();
    //                 cmd.push(ch);
    //                 cmd.push(self.peek());
    //                 cmd.push(self.peek_idx(2));
    //                 cmd.push(self.peek_idx(3));
    //
    //                 if jump_type != Null {
    //                     self.advance();
    //                     self.advance();
    //                     // Reduce line by 1 to match next instruction
    //                     self.line -= 1;
    //                     return Some(Token::Jump(jump_type, self.line));
    //                 }
    //             }
    //             // TODO: handle jump
    //             if self.has_next_idx(2) {
    //                 let mut cmd = String::new();
    //                 cmd.push(ch);
    //                 cmd.push(self.peek());
    //                 cmd.push(self.peek_idx(2));
    //
    //                  let jump_type = match cmd.as_str() {
    //                     "JNG" => JNG,
    //                     "JGT" => JGT,
    //                     "JEQ" => JEQ,
    //                     "JGE" => JGE,
    //                     "JLT" => JLT,
    //                     "JNE" => JNE,
    //                     "JLE" => JLE,
    //                     "JMP" => JMP,
    //                     _ => Null
    //                 };
    //
    //                 if jump_type != Null {
    //                     self.advance();
    //                     self.advance();
    //                     // Reduce line by 1 to match next instruction
    //                     self.line -= 1;
    //                     return Some(Token::Jump(jump_type, self.line));
    //                 }
    //             }
    //
    //             // TODO: handle variable
    //             let mut variable = String::new();
    //             variable.push(ch);
    //             while self.has_next() && self.peek() >= 'A' && self.peek() <= 'Z'  {
    //                 self.advance();
    //                 variable.push(self.char());
    //             }
    //
    //             let token = match variable.as_str() {
    //                 "A" => Token::ARegister,
    //                 "D" => Token::DRegister,
    //                 "M" => Token::Memory,
    //                 _ => Token::Symbol(variable)
    //             };
    //
    //             Some(token)
    //         },
    //         ch if ch.is_digit(10) => {
    //             let mut symbol = String::new();
    //             symbol.push(ch);
    //
    //             while self.has_next() && self.peek().unwrap().is_digit(10) {
    //                 self.advance();
    //                 symbol.push(self.char());
    //             }
    //
    //             Some(Token::Literal(symbol.parse::<i32>().unwrap()))
    //         },
    //         ch if ch == '/' && self.peek() == '/' => {
    //             let comment_line = if self.has_prev() && self.prev() == '\n' {
    //                 true
    //             } else {
    //                 false
    //             };
    //
    //             self.advance();
    //             while self.has_next() && self.char() != '\n' {
    //                 self.advance();
    //             }
    //
    //             if !comment_line {
    //                 self.line += 1;
    //                 return Some(Token::InstructionEnd);
    //             }
    //
    //             Option::None
    //         },
    //         ch if ch == '\n' => {
    //             self.line +=1;
    //             Some(Token::InstructionEnd)
    //         }
    //         _ => {
    //             println!("Token not matched {}", self.char());
    //             Option::None
    //         }
    //     }
    // }

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

    fn release(&mut self) {
        if !self.has_prev() {
            panic!("Out of range");
        }

        self.current_index -= 1;
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

