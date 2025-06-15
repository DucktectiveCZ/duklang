mod tokenizer;
mod parser;

use logos::Logos;
use tokenizer::Token;

use std::fs::read_to_string;
use std::io;

fn main() -> io::Result<()> {
    let source = read_to_string("/home/ducktectivecz/rust/duklang/examples/Class.duk")?;

    let mut lexer = Token::lexer(&source);

    while let Some(tok) = lexer.next() {
        if let Ok(token) = tok {
            match token {
                Token::Ident => println!("Ident: {}", lexer.slice()),
                Token::IntLiteral => println!("IntLiteral: {}", lexer.slice()),
                Token::StrLiteral => println!("StrLiteral: {}", lexer.slice()),
                t => println!("{:?}", t),
            }
        } else {
            eprintln!("Compile time error: {:?}", tok.unwrap_err());
        }
    }

    Ok(())
}
