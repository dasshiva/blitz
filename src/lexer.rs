use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Token {
  IDENT(String),
  OASSIGN,
  NUMBER(i64),
  STRING(String),
  KW_INT,
  DECIMAL(f64),
  EOF,
}

impl Token {
  pub fn new(buf: String) -> Self {
    let trim = buf.trim();
    match trim {
      "int" => return Token::KW_INT,
      "=" => return Token::OASSIGN,
       _  => {}
    }
    match i64::from_str_radix(trim, 10) {
      Ok(s) => return Token::NUMBER(s),
      Err(..) => {
        match f64::from_str(trim) {
          Ok(f) => return Token::DECIMAL(f),
          Err(..) => return Token::IDENT(trim.to_owned())
        }
      }
    }
  }
}

pub struct Lexer<'a> {
  line: usize,
  pos: usize,
  input: &'a [u8],
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a [u8]) -> Self {
    Self {
      input,
      pos: 0,
      line: 1
    }
  }
  
  pub fn next(&mut self) -> Token {
    let mut buf = String::new();
    while self.pos < self.input.len() {
      let c = self.input[self.pos] as char;
      match c {
        ' ' | '\n' => {
          self.line += 1;
          if buf.len() != 0 {
            self.pos += 1;
            return Token::new(buf);
          }
        }
        _ => buf.push(c)
      }
      self.pos += 1;
    }
    
    Token::EOF
  }
}

