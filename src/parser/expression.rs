use crate::parser::tokenizer::Token;
use std::collections::HashMap;

lazy_static! {
    static ref HACK_COMP_MAP: HashMap<&'static str, &'static str> = [
        ("0", "101010"),
        ("1", "111111"),
        ("-1", "111010"),
        ("D", "001100"),
        ("A", "110000"),
        ("M", "110000"),
        ("!D", "001101"),
        ("!A", "110001"),
        ("!M", "110001"),
        ("-D", "001111"),
        ("-A", "110011"),
        ("-M", "110011"),
        ("D+1", "011111"),
        ("A+1", "110111"),
        ("M+1", "110111"),
        ("D-1", "001110"),
        ("A-1", "110010"),
        ("M-1", "110010"),
        ("D+A", "000010"),
        ("D+M", "000010"),
        ("A+D", "000010"),
        ("M+D", "000010"),
        ("D-A", "010011"),
        ("D-M", "010011"),
        ("A-D", "000111"),
        ("M-D", "000111"),
        ("D&A", "000000"),
        ("D&M", "000000"),
        ("D|A", "010101"),
        ("D|M", "010101")
    ].iter().cloned().collect();
    static ref HACK_DEST_MAP: HashMap<&'static str, &'static str> = [
        ("M" , "001"),
        ("D", "010"),
        ("MD", "011"),
        ("A", "100"),
        ("AM", "101"),
        ("AD", "110"),
        ("AMD", "111"),
    ].iter().cloned().collect();
    static ref HACK_JMP_MAP: HashMap<&'static str, &'static str> = [
        ("JNG", "000"),
        ("JGT", "001"),
        ("JEQ", "010"),
        ("JGE", "011"),
        ("JLT", "100"),
        ("JNE", "101"),
        ("JLE", "110"),
        ("JMP", "111"),
    ].iter().cloned().collect();
}

struct AsmCommandDescriptor<'a> {
    comp: &'a str,
    dest: &'a str,
    jump: &'a str,
}

impl<'a> AsmCommandDescriptor<'_> {
    pub fn new() -> AsmCommandDescriptor<'a> {
        AsmCommandDescriptor {
            comp: "",
            dest: "",
            jump: ""
        }
    }

    pub fn into_hack(self) -> String {
        let raw_comp = HACK_COMP_MAP.get(self.comp).unwrap();
        let raw_dest = if let Some(x) = HACK_DEST_MAP.get(self.dest) { x } else {"000"};
        let raw_jump = HACK_JMP_MAP.get(self.jump).unwrap();
        let a = if self.comp.contains("M") { 1 } else { 0 };
        String::from(format!("111{}{}{}{}", a, raw_comp, raw_dest, raw_jump))
    }
}

pub enum  ExpressionType {
    ACommand,
    CCommand,
    JCommand
}

pub struct Expression {
    pub e_type: ExpressionType,
    pub tokens: Vec<Token>
}

impl Expression {
    pub fn new(e_type: ExpressionType, tokens: Vec<Token>) -> Expression {
        Expression {
            e_type,
            tokens
        }
    }
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
                let mut command = AsmCommandDescriptor::new();
                for token in &self.tokens {
                    if let Token::Destination(x) = token  {
                        command.dest = x.as_str();
                    } else {
                        match token {
                            Token::CCommand(raw_command) => {
                                command.comp = raw_command.as_str();
                                command.jump = "JNG";
                            }
                            Token::Jump(raw) => command.jump = raw.as_str(),
                            _ => {}
                        }
                    }
                }

                return command.into_hack()
            },
            ExpressionType::JCommand => {
                let mut command = AsmCommandDescriptor::new();
                for token in &self.tokens {
                    match token {
                        Token::CCommand(x) => command.comp = x,
                        Token::Jump(x) => command.jump = x,
                        _ => {}
                    }
                }

                return command.into_hack()
            }
        }
        String::new()
    }
}
