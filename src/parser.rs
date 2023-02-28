use crate::lexer::Token;
use crate::file::Handle;

#[derive(Debug, PartialEq)]
pub enum Instruction {
  MOV
}

pub enum Args {
  INT(i64),
  DECIMAL(f64),
  STRING(String),
  REGISTER(u8)
}

pub struct Instr {
  name: Instruction,
  len: usize,
  args: Option<Vec<Args>>
}

pub struct Function {
  name: String,
  ins: Vec<Instr>
}

impl Function {
  pub fn new(func: Token) -> Result<Self, &str> {
    let name = match func {
      Token::IDENT(s) => s,
      _ => return Err("Expected identifier after keyword 'func'");
    }
    Ok(Self {
      name,
      ins: Vec::new()
    })
  }
}

pub struct Parser {
  funcs: Vec<Function>,
  state: u8
}

impl Parser {
  pub fn new() -> Self {
    Self {
      funcs: Vec::new(),
      state: 0u8
    }
  }
  
  pub fn parse(&mut self, target: Vec<Token>) -> Result<(), &str> {
    if target.len() == 0 {
      return Ok(());
    }
    if self.state == 0 {
      match target[0] {
        Token::FUNCSTART => {
          self.state = 2;
          if target.len() < 2 {
            return Err("Expected identifier after keyword 'func'");
          }
          match Function::new(target[1]) {
            Ok(s) => self.funcs.push(s),
            Err(e) => return Err(e)
          }
        }
        _ => return Err("Only functions are allowed at top level")
      }
    }
    Ok(())
  }
}