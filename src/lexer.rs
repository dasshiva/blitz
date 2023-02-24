pub enum Tokens {
  IDENT(String),
  OASSIGN,
  NUMBER(i64),
  STRING(String),
  DECIMAL(f64),
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
      line: 0
    }
  }
}

