#[derive(Debug, PartialEq)]
pub enum Ttype {
  INT,
  IDENT(String),
  OPAR,
  CPAR,
  OBRACE,
  RETURN,
  ICONST(i64),
  SEMICOL,
  CBRACE,
  EOF
}

impl Ttype {
  pub fn new(token: &str) -> Self {
    if token.starts_with("0b") {
      return Self::ICONST(i64::from_str_radix(&token[2..], 2).expect("Invalid binary literal"));
    }
    else if token.starts_with("0x") {
      return Self::ICONST(i64::from_str_radix(&token[2..], 16).expect("Invalid hexadecimal literal"));
    }
    else if token.chars().nth(0).unwrap().is_numeric() {
      return Self::ICONST(i64::from_str_radix(&token, 10).expect("Invalid decimal literal"));
    }
    else {
      match token {
        "int" => Self::INT,

        "return" => Self::RETURN,
        _ => Self::IDENT(token.to_string())
      }
    }
  }
}

#[derive(Debug)]
pub struct Token (pub usize, pub usize, pub Ttype);

pub struct Lexer {
  text: String,
  pos: usize,
  line: usize
}

impl Lexer {
  pub fn new(text: String) -> Self {
    Self {
      text,
      pos : 0,
      line: 1
    }
  }

  pub fn lex(&mut self) -> Token {
    let mut buf = String::new();
    let text = self.text.as_bytes();
    while self.pos < text.len()  {
        let read = text[self.pos] as char;
        self.pos += 1;
        match read {
          ' ' | '\t' => {
            if buf.len() == 0 {
              continue;
            }
            return Token(self.pos, self.line, Ttype::new(&buf));
          }
          '\n' => self.line += 1,
          '{' | '}' | '(' | ')' | ';' => {
            if buf.len() != 0 {
              let tok = Token(self.pos, self.line, Ttype::new(&buf));
              buf.clear();
              self.pos -= 1;
              return tok;
            }
            match read {
              '{' =>  return Token(self.pos, self.line, Ttype::OBRACE),
              '}' => return Token(self.pos, self.line, Ttype::CBRACE),
              '(' => return Token(self.pos, self.line, Ttype::OPAR),
              ')' => return Token(self.pos, self.line, Ttype::CPAR),
              ';' => return Token(self.pos, self.line, Ttype::SEMICOL),
              _ => unreachable!()
            }
          }
          _ => buf.push(read)
        }
        
    }
    Token(self.pos, self.line, Ttype::EOF)
  }

  pub fn expect(&mut self, tok: Ttype) {
    let token = self.lex();
    if token.2 != tok {
      panic!("Expected token {tok:?}");
    } 
  }
}