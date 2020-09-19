#![feature(with_options)]

use std::io;
use std::env;
use std::fs::{File, OpenOptions};
use lib::parser::compiler::compile;
use lib::tests::fixtures::{PONG_ASM};

fn main() -> io::Result<()> {
    // let source = String::from_utf8(load_asm().unwrap()).unwrap();



    println!("{}", '\n'.is_digit(10) || '\n'.is_alphabetic());

    compile(String::from(PONG_ASM));


    // save_hack(buffer);

    Ok(())
}

// fn save_hack(data: String) {
//     let args: Vec<String> = env::args().collect();
//     let name = args.get(2).unwrap();
//
//     let file = OpenOptions::new().write(true).create(true).open(name).unwrap();
//     {
//         let mut writer = BufWriter::new(file);
//         writer.write(data.as_bytes()).unwrap();
//     }
// }
//
// fn load_asm() -> io::Result<Vec<u8>> {
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 3 {
//         return io::Result::Err(Error::new(ErrorKind::InvalidInput, "Incorrect input"));
//     }
//
//     let file = File::open(args.get(1).unwrap())?;
//
//     let mut buffer = vec![];
//     let mut reader = BufReader::new(file);
//     reader.read_to_end(&mut buffer).unwrap();
//
//     Ok(buffer)
// }
