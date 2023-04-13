use std::env;

use tokeniser::{Lexer, Ttype};
mod parser;
mod tokeniser;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Filename!")
  }
  let file = match std::fs::read_to_string(&args[1]) {
    Ok(s) => s,
    Err(e) => panic!("Failed to read {} {}", &args[1], e)
  };
  let mut lexer = Lexer::new(file);
  loop {
    let tok = lexer.lex();
    dbg!(&tok);
    if tok.2 == Ttype::EOF {
      break;
    }
  }
}