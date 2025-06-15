mod lexer;
mod parser;

use lexer::Token;
use logos::Logos;

use log::{info, trace};

use std::fs::read_to_string;
use std::io;

fn main() -> io::Result<()> {
    env_logger::init();

    let source = read_to_string("/home/ducktectivecz/rust/duklang/examples/Class.duk").unwrap();

    trace!("Lexer...");
    let mut lexer = Token::lexer(&source);

    trace!("Parser...");
    let module = parser::parse_module(&mut lexer).unwrap();

    info!("Module: {:#?}", module);

    Ok(())
}
