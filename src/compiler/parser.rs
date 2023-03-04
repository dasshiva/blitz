use crate::r#proc::Token;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
  MOV = 1,
  ADD = 2,
  SUB = 3,
  MUL = 4,
  DIV = 5,
  MOD = 6,
  OR = 7,
  AND = 8,
  XOR = 9,
  SHL = 10,
  SHR = 11,
  JMP = 12,
  IFEQ = 13,
  IFNE = 14,
  IFGE = 15,
  IFGT = 16,
  IFLE = 17,
  IFLT = 18
}

impl Instruction {
  pub fn new(name: String) -> Result<(Self, usize), &'static str> {
    match name.as_ref() {
      "mov" | "MOV" => Ok((Instruction::MOV, 3)),
      "add" | "ADD" => Ok((Instruction::ADD, 4)),
      "sub" | "SUB" => Ok((Instruction::SUB, 4)),
      "mul" | "MUL" => Ok((Instruction::MUL, 4)),
      "div" | "DIV" => Ok((Instruction::DIV, 4)),
      "mod" | "MOD" => Ok((Instruction::MOD, 4)),
      "or"  | "OR"  => Ok((Instruction::OR, 4)),
      "and" | "AND" => Ok((Instruction::AND, 4)),
      "xor" | "XOR" => Ok((Instruction::XOR, 4)),
      "shl" | "SHL" => Ok((Instruction::SHL, 4)),
      "shr" | "SHR" => Ok((Instruction::SHR, 4)),
      "jmp" | "JMP" => Ok((Instruction::JMP, 2)),
      "ifeq" | "IFEQ" => Ok((Instruction::IFEQ, 3)),
      "ifne" | "IFNE" => Ok((Instruction::IFNE, 3)),
      "ifge" | "IFGE" => Ok((Instruction::IFGE, 3)),
      "ifgt" | "IFGT" => Ok((Instruction::IFGT, 3)),
      "ifle" | "IFLE" => Ok((Instruction::IFLE, 3)),
      "iflt" | "IFLT" => Ok((Instruction::IFLT, 3)),
      _ => Err("Invalid instruction")
    }
  }
  
  pub fn is_farg_nreg(&self) -> bool {
    match self {
      Instruction::JMP | Instruction::IFEQ | Instruction::IFNE |
      Instruction::IFLT | Instruction::IFLE | Instruction::IFGT |
      Instruction::IFGE => true,
      _ => false
    }
  }
}

#[derive(PartialEq, Debug)]
pub enum Args {
  INT(i64),
  DECIMAL(f64),
  STRING(String),
  LABEL(String),
  REGISTER(u8)
}

impl Args {
  pub fn new(token: &Token, defines: &Vec<Define>) -> Result<Self, &'static str> {
    match token {
      Token::IDENT(s) => {
        if s.starts_with("r") {
          let id = match u8::from_str_radix(&s[1..], 10) {
            Ok(s) => s,
            Err(..) => return Err("Invalid register")
          };
          if id <= 30 {
            return Ok(Args::REGISTER(id));
          }
          return Err("Invalid register number");
        }
        for def in defines {
          if s == &def.0 {
            return Args::new(&def.1, defines);
          }
        }
        return Ok(Args::LABEL(s.to_string()));
      }
      Token::INT(i) => Ok(Args::INT(*i)),
      Token::DECIMAL(j) => Ok(Args::DECIMAL(*j)),
      Token::STRING(str) => Ok(Args::STRING(str.to_owned())),
      _ => Err("Illegal argument to instruction")
    }
  }
}

#[derive(Debug)]
pub struct Instr {
  pub name: Instruction,
  pub len: usize,
  pub has_label: bool,
  pub args: Option<Vec<Args>>
}

impl Instr {
  pub fn new(name: String) -> Result<Self, &'static str> {
    let (ins, len) = Instruction::new(name)?;
    Ok(Self {
      name: ins,
      has_label: false,
      len,
      args: {
        if len != 1 {
          Some(Vec::<Args>::new())
        }
        else {
          None
        }
      }
    })
  }
  
  pub fn add_args(&mut self, args: &[Token], defines: &Vec<Define>) -> Result<(), &'static str> {
    if args.len() + 1 != self.len {
      return Err("Instruction has been given more or less arguments than needed");
    }
    for arg in args {
      let ar = Args::new(arg, defines)?;
      match ar {
        Args::LABEL(..) => self.has_label = true,
        _ => {}
      }
      self.args.as_mut().unwrap().push(ar);
    }
    if !self.name.is_farg_nreg() {
      match self.args.as_ref().unwrap()[0] {
        Args::REGISTER(..) => {},
        _ => return Err("First argument to instruction must be a register")
      }
    }
    Ok(())
  }
}

pub struct Function {
 pub name: String,
 pub ins: Vec<Instr>
}

impl Function {
  pub fn new(func: &Token) -> Result<Self, &'static str> {
    let name = match func {
      Token::IDENT(s) => s.to_string(),
      _ => return Err("Expected identifier after keyword 'func'")
    };
    Ok(Self {
      name,
      ins: Vec::new()
    })
  }
  
  pub fn add_ins(&mut self, ins: Instr) {
    self.ins.push(ins)
  }
}

pub struct Define(String, Token);

pub struct Parser {
  define: Vec<Define>,
  labels: Vec<Define>,
  pub funcs: Vec<Function>,
  state: u8
}

impl Parser {
  pub fn new() -> Self {
    Self {
      define: Vec::new(),
      funcs: Vec::new(),
      labels: Vec::new(),
      state: 0u8
    }
  }
  
  pub fn parse(&mut self, target: Vec<Token>) -> Result<(), &str> {
    if target.len() == 0 {
      return Ok(());
    }
    if self.state == 0 {
      match target[0] {
        Token::FUNC => {
          self.state = 2;
          if target.len() < 2 {
            return Err("Expected identifier after keyword 'func'");
          }
          match Function::new(&target[1]) {
            Ok(s) => self.funcs.push(s),
            Err(e) => return Err(e)
          }
        }
        Token::DEFINE => {
          if target.len() < 3 {
            return Err("define keyword has to be followed by an identifier and a value");
          }
          let name = match &target[1] {
            Token::IDENT(s) => s.to_string(),
            _ => return Err("Expected identifier after define keyword")
          };
          match &target[2] {
            Token::INT(..) | Token::DECIMAL(..) | Token::STRING(..) | Token::IDENT(..) => self.define.push(Define(name, target[2].clone())),
            _ => return Err("Expected value here")
          }
        }
        _ => return Err("Only functions are allowed at top level")
      }
    }
    else if self.state == 2 {
      match &target[0] {
        Token::IDENT(s) => {
          let mut this_func = self.funcs.pop().unwrap();
          let mut ins = Instr::new(s.to_owned())?;
          if ins.len != 1 {
            ins.add_args(&target[1..], &self.define)?;
          }
          this_func.add_ins(ins);
          self.funcs.push(this_func);
        },
        Token::LABEL(s) => {
          let func = self.funcs.pop().unwrap();
          self.labels.push(Define(s.to_string(), Token::INT(func.ins.len() as i64)));
          self.funcs.push(func);
        }
        Token::ENDFUNC => {
          self.state = 0;
          let mut this_func = self.funcs.pop().unwrap();
          for ins in &mut this_func.ins {
            if ins.has_label {
              let args = ins.args.as_mut().unwrap();
              for i in 0..args.len() {
                match &args[i] {
                  Args::LABEL(s) => {
                    let label = self.find_label(&s)?;
                    args[i] = match self.labels[label].1 {
                      Token::INT(s) => Args::INT(s),
                      _ => unreachable!()
                    };
                  } 
                  _ => {}
                } // match args[i]
              } // args_loop
            } // if ins.has_label
          } // ins_loop
          self.funcs.push(this_func);
          self.labels.clear();
        } // match TOKEN::ENDFUNC
        _ => return Err("Expected instruction name here")
      }
    }
    Ok(())
  }
  
  fn find_label(&self, name: &str) -> Result<usize, &'static str> {
    for i in 0..self.labels.len() {
      if self.labels[i].0 == name {
        return Ok(i);
      }
    }
    Err("Label not found")
  }
}
