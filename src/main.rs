#![feature(with_options)]

use std::io;
use std::env;
use std::str;
use std::char;
use std::fs::{File, OpenOptions};
use std::io::{Read, BufReader, BufRead, ErrorKind, Error, BufWriter, Write};
use lib::parser::parser::{Parser, Expression, Evaluate};
use lib::parser::tokenizer::Tokenizer;
use std::ops::Add;

fn main() -> io::Result<()> {

    let r = String::from_utf8(load_asm().unwrap()).unwrap();
    let tokens = Tokenizer::new().tokenize(r);
    let expressions = Parser::new().parse(&tokens);
    let mut buffer = String::new();
    for expression in expressions {
        if buffer.len() > 0 {
            buffer.push_str("\r\n");
        }
        let res = expression.evaluate();
        buffer.push_str(res.as_str());
    }

    save_hack(buffer);

    Ok(())
}

fn save_hack(data: String) {
    let args: Vec<String> = env::args().collect();
    let name = args.get(2).unwrap();

    let mut file = OpenOptions::new().write(true).create(true).open(name).unwrap();
    {
        let mut writer = std::io::BufWriter::new(file);
        writer.write(data.as_bytes());
    }
}

fn load_asm() -> io::Result<Vec<u8>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return io::Result::Err(Error::new(ErrorKind::InvalidInput, "Incorrect input"));
    }

    let file = File::open(args.get(1).unwrap())?;

    let mut buffer = vec![];
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut buffer);

    Ok(buffer)
}
