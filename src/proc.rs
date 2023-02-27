use crate::file::Handle;
use std::str::FromStr;
// This lexer and parser have one inherent limitation
// They cannot process instructions extended over more than one line

#[derive(Debug)]
pub enum Instruction {
  MOV
}

#[derive(Debug)]
pub enum Token {
  INS(Instruction),
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
      "mov" => return Token::INS(Instruction::MOV),
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
      ':' | ',' => {} // Ignore commaand and semi colon
      _ => buf.push(c)
    }
  }
  if instr {
    return Err("Unclosed string literal");
  }
  Ok(ret)
}

pub struct Unit {
  
}

impl Unit {
  pub fn new(mut src: Handle) -> Unit {
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
      println!("{:?}", split);
    }
    Unit {
      
    }
  }
}