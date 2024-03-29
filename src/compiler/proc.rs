use crate::file::Handle;
use crate::parser::*;
use std::str::FromStr;
// This lexer and parser have one inherent limitation
// They cannot process instructions extended over more than one line

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  FUNC,
  DEFINE,
  INCLUDE,
  ENDFUNC,
  INT(i64),
  DECIMAL(f64),
  STRING(String),
  IDENT(String),
  LABEL(String),
  ATTR(String),
  PLUS,
  MINUS,
  OFFSET(String)
}

impl Token {
  pub fn new(token: &str) -> Self {
    let first = token.chars().nth(0);
    if token.starts_with("0x") {
      match i64::from_str_radix(&token[2..], 16) {
        Ok(s) => return Token::INT(s),
        Err(..) => panic!("Malformed hexadecimal integer")
      }
    }
    if token.starts_with("0b") {
      match i64::from_str_radix(&token[2..], 2) {
        Ok(s) => return Token::INT(s),
        Err(..) => panic!("Malformed binary integer")
      }
    }
    else if first == Some('\'') || first == Some('\"') {
      return Token::STRING(token[1..].to_owned());
    }
    else if first == Some('.') {
      return Token::ATTR(token[1..].to_owned());
    }
    else if first == Some('[') {
      return Token::OFFSET(token[1..].to_owned());
    }
    if token.chars().nth(token.len() - 1) == Some(':') {
      return Token::LABEL(token[0..token.len()-1].to_string());
    }
    match token {
      "func" => return Token::FUNC,
      "end" => return Token::ENDFUNC,
      "define" => return Token::DEFINE,
      "include" => return Token::INCLUDE,
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

pub fn line_split(string: &[u8]) -> Result<Vec<Token>, &str> {
  let mut ret: Vec<Token> = Vec::new();
  let mut instr = false;
  let mut comm = false;
  let mut attr = false;
  let mut inexpr = false;
  let mut buf = String::new();
  for ch in string {
    let c = *ch as char;
    match c {
      ' ' | '\n' => {
        if c == ' ' && attr {
          if buf.len() == 0 {
            return Err("Attribute name expected");
          }
          attr = false;
          ret.push(Token::new(&buf));
          buf.clear();
          continue;
        }
        if buf.len() == 0 {
          if c == ' ' {
            continue;
          }
          break;
        }
        if instr || inexpr {
          buf.push(c);
          continue;
        }
        ret.push(Token::new(&buf));
        if c == '\n' {
          break;
        }
        buf.clear();
      }
      '.' => {
        if buf.len() != 0 {
          buf.push(c);
          continue;
        }
        buf.push(c);
        attr = true;
      }
      '[' => {
        if inexpr {
          panic!("Nested offsets are not allowed");
        }
        if buf.len() != 0 {
            ret.push(Token::new(&buf));
        }
        inexpr = true;
        buf.push(c);
      }
      '+' => {
        if inexpr {
          buf.push(c);
          continue;
        }
        if buf.len() != 0 {
            ret.push(Token::new(&buf));
        }
        ret.push(Token::PLUS);
      }
      '-' => {
        if inexpr {
          buf.push(c);
          continue;
        }
        if buf.len() != 0 {
            ret.push(Token::new(&buf));
        }
        ret.push(Token::MINUS);
      }
      ']' => {
        inexpr = false;
        if buf.len() == 0 {
          panic!("Empty offset");
        }
        ret.push(Token::new(&buf));
        buf.clear();
        //println!("{ret:?}");
      }
      '/' => {
        if comm {
          if buf.len() != 0 {
            ret.push(Token::new(&buf));
          }
          return Ok(ret);
        }
        comm = true;
      }
      '\'' | '\"' => {
        if instr {
          ret.push(Token::new(&buf));
          buf.clear();
          instr = false;
          continue;
        }
        instr = true;
        buf.push(c);
      }
      ';' | ',' => {} // Ignore comma and and semi colon
      _ => buf.push(c)
    }
  }
  if instr {
    return Err("Unclosed string literal");
  }
  if comm {
    return Err("Unclosed comment");
  }
  if inexpr {
    return Err("Unclosed offset");
  }
  Ok(ret)
}

pub struct Unit {
  pub name: String,
  pub funcs: Vec<Function>,
  pub define: Vec<Define>,
  pub data: Vec<Data>
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
    let split = match line_split("include \"firmware.su\" ".as_bytes()) {
        Ok(s) => s,
        Err(e) => src.error(e)
    };
    match parser.parse(split) {
      Ok(..) => {},
      Err(e) => src.error(e)
    }
    Unit {
      name: src.file,
      funcs: parser.funcs,
      define: parser.define,
      data: parser.data
    }
  }
}
