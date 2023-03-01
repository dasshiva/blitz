use crate::file::Handle;
use crate::parser::*;
use std::str::FromStr;
use crate::serde::{Serialize, Deserialize};
extern crate alloc;
extern crate postcard;
use alloc::vec::Vec;
use postcard::to_allocvec;
use std::fs::File;
#[cfg(target_family = "unix")]
use std::os::unix::fs::FileExt;
#[cfg(target_family = "windows")]
use std::os::windows::fs::FileExt;
// This lexer and parser have one inherent limitation
// They cannot process instructions extended over more than one line

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  FUNC,
  DEFINE,
  ENDFUNC,
  INT(i64),
  DECIMAL(f64),
  STRING(String),
  IDENT(String)
}

impl Token {
  pub fn new(token: &str) -> Self {
    if token.chars().nth(0) == Some('\'') {
      return Token::STRING(token[1..].to_owned());
    }
    match token {
      "func" => return Token::FUNC,
      "end" => return Token::ENDFUNC,
      "define" => return Token::DEFINE,
       _ => {}
    }
    match i64::from_str_radix(token, 10) {
      Ok(s) => return Token::INT(s),
      Err(..) => {
        match f64::from_str(token) {
          Ok(f) => return Token::DECIMAL(f),
          Err(..) => return Token::IDENT(token.to_owned())
        }
      }
    }
  }
}

fn line_split(string: &[u8]) -> Result<Vec<Token>, &str> {
  let mut ret: Vec<Token> = Vec::new();
  let mut instr = false;
  let mut buf = String::new();
  for ch in string {
    let c = *ch as char;
    match c {
      ' ' | '\n' => {
        if buf.len() == 0 || instr {
          buf.push(c);
          continue;
        }
        ret.push(Token::new(&buf));
        if c == '\n' {
          break;
        }
        buf.clear();
      }
      '\'' => {
        if instr {
          ret.push(Token::new(&buf));
          buf.clear();
          instr = false;
          continue;
        }
        instr = true;
        buf.push(c);
      }
      ':' | ',' => {} // Ignore comma and and semi colon
      _ => buf.push(c)
    }
  }
  if instr {
    return Err("Unclosed string literal");
  }
  Ok(ret)
}

#[derive(Serialize, Deserialize)]
pub struct Unit {
  name: String,
  funcs: Vec<Function>
}

impl Unit {
  pub fn new(mut src: Handle) -> Unit {
    let mut parser = Parser::new();
    loop {
      let mut line = src.read_line();
      if &line == "EOF" {
        break;
      }
      line.push(' ');
      let split = match line_split(line.as_bytes()) {
        Ok(s) => s,
        Err(e) => src.error(e)
      };
      match parser.parse(split) {
        Ok(..) => {},
         Err(e) => src.error(e)
      }
    }
    Unit {
      name: src.file,
      funcs: parser.funcs
    }
  }
  
  pub fn gen(&self) {
    let file = File::create(self.name.clone() + ".out").expect("Failed to open output file");
    let output: Vec<u8> = to_allocvec(self).unwrap();
    #[cfg(target_family = "unix")]
    file.write_at(&output, 0);
    #[cfg(target_family = "windows")]
    file.seek_read(&output, 0);
  }
}
