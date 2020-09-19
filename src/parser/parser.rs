use crate::parser::tokenizer::{Token, OperationType};
use std::collections::HashMap;
use crate::parser::tokenizer::Token::{Symbol, JumpSymbol};
use std::borrow::Borrow;
use std::str;

pub struct Parser {
    sym_table: SymTable
}

pub enum  ExpressionType {
    ACommand,
    CCommand,
    JCommand
}

pub struct Expression {
    e_type: ExpressionType,
    tokens: Vec<Token>
}

pub trait Evaluate {
    fn evaluate(&self) -> String;
}

impl Evaluate for Expression {
    fn evaluate(&self) -> String {
        match self.e_type {
            ExpressionType::ACommand => {
                for token in &self.tokens {
                    if let Token::ACommandLiteral(x) = token {
                        return format!("0{:015b}", x);
                    } else {
                        panic!("Evaluation error")
                    }
                }
            },
            ExpressionType::CCommand => {
                let mut dest = String::from("000");
                let mut comp = String::new();
                let mut a: i32 = 0;
                let mut jump = "000";

                for token in &self.tokens {
                    if let Token::Destination(x) = token {
                        dest = match x.clone().as_str() {
                            "M" => "001",
                            "D" => "010",
                            "MD" => "011",
                            "A" => "100",
                            "AM" => "101",
                            "AD" => "110",
                            "AMD" => "111",
                            _ => "000"
                        }.to_string();
                    } else {
                        match token {
                            Token::CCommand(x) => {
                                comp = match x.clone().as_str() {
                                    "0" => "101010",
                                    "1" => "111111",
                                    "-1" => "111010",
                                    "D" => "001100",
                                    "A" | "M" => "110000",
                                    "!D" => "001101",
                                    "!A" | "!M" => "110001",
                                    "-D" => "001111",
                                    "-A" | "-M" => "110011",
                                    "D+1" => "011111",
                                    "A+1" | "M+1" => "110111",
                                    "D-1" => "001110",
                                    "A-1" | "M-1" => "110010",
                                    "D+A" | "D+M" => "000010",
                                    "A+D" | "M+D" => "000010",
                                    "D-A" | "D-M" => "010011",
                                    "A-D" | "M-D" => "000111",
                                    "D&A" | "D&M" => "000000",
                                    "D|A" | "D|M" => "010101",
                                    _ => {
                                        let cx = 123;
                                        panic!("Comp parse error");
                                    }
                                }.to_string();
                                a = if x.contains("M") {
                                    1
                                } else { 0 };
                            },
                            Token::Jump(x) => {
                                jump = match x.as_str() {
                                    "JNG" => "000",
                                    "JGT" => "001",
                                    "JEQ" => "010",
                                    "JGE" => "011",
                                    "JLT" => "100",
                                    "JNE" => "101",
                                    "JLE" => "110",
                                    "JMP" => "111",
                                    _ => panic!("Token not recognized")
                                };
                            }
                            _ => {}
                        }


                    }
                }
                return String::from(format!("111{}{}{}{}", a, comp, dest, jump));
            },
            ExpressionType::JCommand => {
                let mut comp = String::new();
                let mut jump = String::new();
                let mut dest = String::from("000");
                let mut a = 0;
                for token in &self.tokens {
                    match token {
                        Token::CCommand(x) => {
                            comp = match x.clone().as_str() {
                                "0" => "101010",
                                "1" => "111111",
                                "-1" => "111010",
                                "D" => "001100",
                                "A" | "M" => "110000",
                                "!D" => "001101",
                                "!A" | "!M" => "110001",
                                "-D" => "001111",
                                "-A" | "-M" => "110011",
                                "D+1" => "011111",
                                "A+1" | "M+1" => "110111",
                                "D-1" => "001110",
                                "A-1" | "M-1" => "110010",
                                "D+A" | "D+M" => "000010",
                                "A+D" | "M+D" => "000010",
                                "D-A" | "D-M" => "010011",
                                "A-D" | "M-D" => "000111",
                                "D&A" | "D&M" => "000000",
                                "D|A" | "D|M" => "010101",
                                _ => {
                                    panic!("Comp parse error");
                                }
                            }.to_string();
                            a = if x.contains("M") {
                                1
                            } else { 0 };
                        },
                        Token::Jump(x) => {
                            let j = match x.clone().as_str() {
                                "JNG" => "000",
                                "JGT" => "001",
                                "JEQ" => "010",
                                "JGE" => "011",
                                "JLT" => "100",
                                "JNE" => "101",
                                "JLE" => "110",
                                "JMP" => "111",
                                _ => panic!("Token not recognized")
                            };
                            jump = String::from(j);
                        }
                        _ => {}
                    }
                }
                return String::from(format!("111{}{}{}{}", a, comp, dest, jump));
            },
            _ => {}
        }
        String::new()
    }
}


impl Parser {
    pub fn new() -> Parser {
        Parser {
            sym_table: SymTable::new()
        }
    }

    pub fn parse(&mut self, tokens: &Vec<Token>) -> Vec<Expression> {
        self.register_symbols(tokens);
        let mut i = 0;
        let mut expressions: Vec<Expression> = vec![];
        while i < tokens.len() {
            match tokens.get(i).unwrap() {
                Token::ACommandSymbol(s) => {
                    if let Some(e) = self.sym_table.get(s.clone()) {
                        let e = self.sym_table.get(s.clone()).unwrap();
                        expressions.push(Expression {
                            e_type: ExpressionType::ACommand,
                            tokens: vec![Token::ACommandLiteral(*e)]
                        });
                        i+=1;
                    } else {
                        println!("symbol `{}`", s);
                        let x = 123;
                        panic!("Invalid symbol");
                    }
                },
                Token::ACommandLiteral(e) => {
                    expressions.push(Expression {
                        e_type: ExpressionType::ACommand,
                        tokens: vec![Token::ACommandLiteral(*e)]
                    });
                    i+=1;
                },
                Token::Destination(s) => {
                    i+=1;
                    if let Token::CCommand(x) = tokens.get(i).unwrap() {
                        expressions.push(Expression {
                            e_type: ExpressionType::CCommand,
                            tokens: vec![Token::Destination(s.clone()), Token::CCommand(x.clone())]
                        });
                        i+=1;
                    } else {
                        panic!("CCommand should follow after Destination command")
                    }
                },
                // Were in jump
                Token::CCommand(x) => {
                    let ccomand = x;
                    i+=1;
                    if let Token::Jump(x) = tokens.get(i).unwrap() {
                        expressions.push(Expression {
                            e_type: ExpressionType::JCommand,
                            tokens: vec![Token::CCommand(ccomand.clone()), Token::Jump(x.clone())]
                        });
                        i+=1;
                    } else {
                        panic!("Jump should follow after CCcomand")
                    }

                },
                Token::InstructionEnd | Token::JumpSymbol(_, _) => {
                    i+=1;
                },
                _ => {
                    let t = tokens.get(i).unwrap();
                    panic!("Unexpected token")
                }
            }
        }

        expressions
    }

    fn register_symbols(&mut self, tokens: &Vec<Token>) {
        for token in tokens {
            match token {
                Symbol(x) => {
                    self.sym_table.add(x.clone());
                },
                JumpSymbol(x, address) => {
                    self.sym_table.set(x.clone(), *address);
                },
                _ => {}
            }
        }
    }
}

struct SymTable {
    entries: HashMap<String, u32>,
    address: u32
}


impl SymTable{
    pub fn new() -> SymTable {
        let mut  table = SymTable {
            entries: HashMap::new(),
            address: 16
        };
        table.add_defaults();
        table
    }

    fn add_defaults(&mut self) {
        self.set(String::from("SP"), 0);
        self.set(String::from("LCL"), 1);
        self.set(String::from("ARG"), 2);
        self.set(String::from("THIS"), 3);
        self.set(String::from("THAT"), 4);
        self.set(String::from("R0"), 0);
        self.set(String::from("R1"), 1);
        self.set(String::from("R2"), 2);
        self.set(String::from("R3"), 3);
        self.set(String::from("R4"), 4);
        self.set(String::from("R5"), 5);
        self.set(String::from("R6"), 6);
        self.set(String::from("R7"), 7);
        self.set(String::from("R8"), 8);
        self.set(String::from("R9"), 9);
        self.set(String::from("R10"), 10);
        self.set(String::from("R11"), 11);
        self.set(String::from("R12"), 12);
        self.set(String::from("R13"), 13);
        self.set(String::from("R14"), 14);
        self.set(String::from("R15"), 15);
        self.set(String::from("SCREEN"), 16384);
        self.set(String::from("KBD"), 24576);
    }

    pub fn get(&self, key: String) -> Option<&u32> {
        self.entries.get(&key)
    }

    pub fn add(&mut self, key: String) {
        self.set(key, self.address);
        self.address += 1;
    }

    fn set(&mut self, key: String, value: u32) {
        self.entries.insert(key, value);
    }
}