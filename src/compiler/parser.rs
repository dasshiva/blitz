use crate::r#proc::{Token, line_split};
use crate::file::Handle;
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
  NOP = 0,
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
  IFLT = 18,
  CALL = 19,
  FMOV = 20,
  FADD = 21,
  FSUB = 22,
  FMUL = 23,
  FDIV = 24,
  FMOD = 25,
  FIFEQ = 26,
  FIFNE = 27,
  FIFGE = 28,
  FIFGT = 29,
  FIFLE = 30,
  FIFLT = 31,
  FPUSH = 32,
  FPOP = 33,
  PUSH = 34,
  POP = 35,
  LEA = 36
}

impl Instruction {
  pub fn new(name: String) -> Result<(Self, usize), &'static str> {
    match name.as_ref() {
      "nop" | "NOP" => Ok((Instruction::NOP, 1)),
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
      "call" | "CALL" => Ok((Instruction::CALL, 2)),
      "fmov" | "FMOV" => Ok((Instruction::FMOV, 3)),
      "fadd" | "FADD" => Ok((Instruction::FADD, 4)),
      "fsub" | "FSUB" => Ok((Instruction::FSUB, 4)),
      "fmul" | "FMUL" => Ok((Instruction::FMUL, 4)),
      "fdiv" | "FDIV" => Ok((Instruction::FDIV, 4)),
      "fmod" | "FMOD" => Ok((Instruction::FMOD, 4)),
      "fifeq" | "FIFEQ" => Ok((Instruction::FIFEQ, 3)),
      "fifne" | "FIFNE" => Ok((Instruction::FIFNE, 3)),
      "fifge" | "FIFGE" => Ok((Instruction::FIFGE, 3)),
      "fifgt" | "FIFGT" => Ok((Instruction::FIFGT, 3)),
      "fifle" | "FIFLE" => Ok((Instruction::FIFLE, 3)),
      "fiflt" | "FIFLT" => Ok((Instruction::FIFLT, 3)),
      "fpush" | "FPUSH" => Ok((Instruction::FPUSH, 2)),
      "fpop" | "FPOP" => Ok((Instruction::FPOP, 2)),
      "push" | "PUSH" => Ok((Instruction::PUSH, 2)),
      "pop" | "POP" => Ok((Instruction::POP, 2)),
      "lea" | "LEA" => Ok((Instruction::LEA, 3)),
      _ => Err("Invalid instruction")
    }
  }
  
  pub fn is_farg_nreg(&self) -> bool {
    match self {
      Instruction::JMP | Instruction::IFEQ | Instruction::IFNE |
      Instruction::IFLT | Instruction::IFLE | Instruction::IFGT |
      Instruction::IFGE | Instruction::CALL | Instruction::PUSH | Instruction::FPUSH | Instruction::MOV => true,
      _ => false
    }
  }
  
  pub fn is_no_arg(&self) -> bool {
    match self {
      Instruction::NOP => true,
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
  REGISTER(u8),
  OFFSET(u8, i64),
}

impl Args {
  pub fn new(token: &Token, defines: &Vec<Define>) -> Result<Self, &'static str> {
    match token {
      Token::IDENT(s) => {
        if s.starts_with("r") || s.starts_with('f') {
          let id = match u8::from_str_radix(&s[1..], 10) {
            Ok(s) => s,
            Err(..) => return Err("Invalid register")
          };
          if id <= 30 {
            return Ok(Args::REGISTER(id));
          }
          return Err("Invalid register number");
        }
        else if s == "sp" {
          return Ok(Args::REGISTER(31));
        }
        for def in defines {
          if s == &def.0 {
            return Args::new(&def.1, defines);
          }
        }
        return Ok(Args::LABEL(s.to_string()));
      }
      Token::OFFSET(s) => {
        let mut arg = s.to_string();
        arg.push(' ');
        let split = match line_split(&arg.as_bytes()) {
          Ok(s) => s,
          Err(..) => unreachable!()
        };
        let mut reg = 0u8;
        match &split[0] {
          Token::IDENT(i) => {
            match Args::new(&split[0], defines)? {
              Args::REGISTER(r) => reg = r,
              _ => return Err("First argument to offset must be register")
            }
          }
          _ => return Err("First argument to offset must be register")
        }
        
        if split.len() == 1 {
          return Ok(Args::OFFSET(reg, 0));
        }
        
        if split.len() < 3 {
          panic!("Offset argument incomplete");
        }
        
        let number = match &split[2] {
          Token::INT(s) => *s,
          _ => panic!("Offset must be int")
        };
        match &split[1] {
          Token::PLUS => {
            return Ok(Args::OFFSET(reg, number));
          }
          Token::MINUS => {
            return Ok(Args::OFFSET(reg, -number));
          }
          _ => return Err("Only operator + or - allowed in offset")
        }
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
        Args::LABEL(..) => {
        println!("{arg:?}");
        self.has_label = true;
        }
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

pub enum Attrs {
  FIRMWARE
}

pub struct Attr(pub Attrs, pub Option<Token>);
impl Attr {
  pub fn file_new(name: &str, arg: Option<Token>) -> Result<Self, &'static str> {
    match name {
      "firmware" => Ok(Self(Attrs::FIRMWARE, None)),
      _ => Err ("Unknown attribute or attribute not expected here")
    }
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
  pub attrs: Option<Vec<Attr>>,
  pub funcs: Vec<Function>,
  state: u8
}

impl Parser {
  pub fn new() -> Self {
    Self {
      define: Vec::new(),
      funcs: Vec::new(),
      labels: Vec::new(),
      attrs: None,
      state: 0u8
    }
  }
  
  pub fn parse(&mut self, target: Vec<Token>) -> Result<(), &str> {
    if target.len() == 0 {
      return Ok(());
    }
    if self.state == 0 {
      match &target[0] {
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
        Token::ATTR(s) => {
          let attr = Attr::file_new(&s, target.get(1).cloned())?;
          if self.attrs.is_none() {
            let mut vector: Vec<Attr> = Vec::new();
            vector.push(attr);
            self.attrs = Some(vector);
            return Ok(());
          }
          self.attrs.as_mut().unwrap().push(attr);
        }
        Token::INCLUDE => {
          if target.len() < 2 {
            return Err("include keyword has to be followed by a file name");
          }
          let name = match &target[1] {
            Token::STRING(s) => s, 
            _ => return Err("Expected string representing filename after define keyword")
          };
          let mut src = Handle::new(name);
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
          self.define.append(&mut parser.define);
          self.funcs.append(&mut parser.funcs);
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
      println!("{:?}", target[0]);
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
                    match self.find_label(&s) {
                      Ok(s) => {
                        args[i] = match self.labels[s].1 {
                          Token::INT(s) => Args::INT(s),
                          _ => unreachable!()
                        };
                      }
                      Err(..) => {
                        if ins.name != Instruction::CALL {
                          return Err("Label not found");
                        }
                        args[i] = Args::STRING(s.to_string());
                      }
                    }
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
  
  fn find_label(&self, name: &str) -> Result<usize, ()> {
    for i in 0..self.labels.len() {
      if self.labels[i].0 == name {
        return Ok(i);
      }
    }
    Err(())
  }
}
