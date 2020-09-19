use crate::parser::tokenizer::{Token};
use std::collections::HashMap;
use crate::parser::tokenizer::Token::{JumpSymbol, ACommandSymbol};
use crate::parser::expression::{Expression, ExpressionType};

pub struct Parser {
    sym_table: SymTable
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            sym_table: SymTable::new()
        }
    }

    pub fn parse(&mut self, tokens: &Vec<Token>) -> Vec<Expression> {
        self.register_symbols(tokens);
        // let sysinit = self.sym_table.get(String::from("OUTPUT_FIRST")).unwrap();
        // println!("OUTPUT_FIRST {}", sysinit);

        let mut i = 0;
        let mut expressions: Vec<Expression> = vec![];
        while i < tokens.len() {
            match tokens.get(i).unwrap() {
                Token::ACommandSymbol(s) => {
                    if let Some(e) = self.sym_table.get(s.clone()) {
                        expressions.push(Expression {
                            e_type: ExpressionType::ACommand,
                            tokens: vec![Token::ACommandLiteral(*e)]
                        });
                        i+=1;
                    } else {
                        let h = self.sym_table.entries.contains_key(s);
                        let symbol = s.clone();
                        panic!("Invalid symbol");
                    }
                },
                Token::ACommandLiteral(e) => {
                    expressions.push(Expression::new(ExpressionType::ACommand, vec![Token::ACommandLiteral(*e)]));
                    i+=1;
                },
                Token::Destination(s) => {
                    i+=1;
                    if let Token::CCommand(x) = tokens.get(i).unwrap() {
                        let expression = Expression::new(ExpressionType::CCommand, vec![Token::Destination(s.clone()), Token::CCommand(x.clone())]);
                        expressions.push(expression);
                        i+=1;
                    } else {
                        panic!("CCommand should follow after Destination command")
                    }
                },
                // We are in jump
                Token::CCommand(x) => {
                    let c_comand = x;
                    i+=1;
                    if let Some(Token::Jump(x)) = tokens.get(i) {
                        let expression = Expression::new(ExpressionType::JCommand, vec![Token::CCommand(c_comand.clone()), Token::Jump(x.clone())]);
                        expressions.push(expression);
                        i+=1;
                    } else {
                        panic!("Jump should follow after CCcomand")
                    }
                },
                Token::InstructionEnd | Token::JumpSymbol(_, _) => i+=1,
                _ => panic!("Unexpected token")
            }
        }

        expressions
    }

    fn register_symbols(&mut self, tokens: &Vec<Token>) {
        for token in tokens {
            match token {
                JumpSymbol(x, address) => {
                    if !self.sym_table.entries.contains_key(x.as_str()) {
                        self.sym_table.set(x.clone(), *address);
                    } else {
                        println!("****JumpSymbol already added");
                    }
                },
                _ => {}
            }
        }

        for token in tokens {
            match token {
                ACommandSymbol(x) => {
                    if !self.sym_table.entries.contains_key(x.as_str()) {
                        self.sym_table.add(x.clone());
                    }
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