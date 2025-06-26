mod parser;
mod lexer;
mod tests;

use owo_colors::OwoColorize;
use rustyline::{DefaultEditor, Result};
use parser::Parser;

fn main() -> Result<()> {
    env_logger::init();

    let mut rl = DefaultEditor::new()?;

    loop {
        let line = rl.readline(">> ");

        match line {
            Ok(source) => {
                rl.add_history_entry(source.as_str())?;
                let ast_res = Parser::new(&source).parse_fun_decl();
                match ast_res {
                    Ok(ast) => println!("Parsed AST: {:#?}", ast),
                    Err(err) => println!("{}{}", "Syntax error: ".red(), err),
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) |
            Err(rustyline::error::ReadlineError::Eof) => break, // Ctrl+C or Ctrl+D
            Err(err) => {
                eprintln!("Error: {err}");
                break;
            }
        }
    }

    Ok(())
}
