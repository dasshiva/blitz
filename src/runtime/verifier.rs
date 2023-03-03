extern crate file_utils;
use std::io::Error;
use std::fs::File;
use file_utils::read::Read;

fn str_from_bytes(file: &mut File) -> Result<String, Error> {
  let size = file.read_u32()?;
  if size == 0 {
    return String::new();
  }
  let buf: Vec<u8> = Vec::with_capacity(size as usize);
  for _ in 0..size {
    buf.push(file.read_u8()?);
  }
  unsafe {
    Ok(String::from_utf8_unchecked(buf))
  }
}

pub enum Instruction {
  MOV = 1
}

pub enum Args {
  INT(i64),
  DECIMAL(f64),
  STRING(String),
  REGISTER(u8)
}

pub struct Ins {
  name: Instruction,
  sign: String,
  args: Option<Vec<Args>>
}

impl Ins {
  pub fn new(file: &mut File) -> Result<Self, Error> {
    let name = file.read_u16()? as Instruction;
    let sign = str_from_bytes(file)?;
    if sign.len() == 0 {
      Ok({
        name,
        sign,
        args: None
      });
    }
    else {
      let mut args: Vec<Args> = Vec::new();
      for byte in sign.as_bytes() {
        match byte as char {
          'R' => {
            let reg = file.read_u8()?;
            args.push(Args::REGISTER(reg));
          }
        }
      }
    }
  }
}

pub struct Function {
  name: String,
  ins: Vec<Ins>
}

impl Function {
  pub fn new(file: &mut File) -> Result<Self, Error> {
    let name = str_from_bytes(file);
    let ins_size = file.read_u32()?;
    let mut ins: Vec<Ins> = Vec::new();
    for _ in 0..ins_size {
      ins.push(Ins::new(file)?);
    }
    Ok({
      name,
      ins
    })
  }
}

pub struct Unit {
  major: u16,
  minor: u16,
  funcs: Vec<Function>
}

const MAGIC: u32 = 0xAFC;
const MAJOR: u16 = 0x1;
const MINOR: u16 = 0x0;

impl Unit {
  pub fn new(name: &str) -> Result<Self, Error> {
    let mut file = File::open(name)?;
    let magic = file.read_u32()?;
    if magic != MAGIC {
      panic!("File {name} is not a blitz executable");
    }
    let major = file.read_u16()?;
    let minor = file.read_u16()?;
    if major != MAJOR || major != MINOR {
      panic!("Unsupported blitz version {major}.{minor}");
    }
    let func_len = file.read_u32()?;
    let mut funcs: Vec<Function> = Vec::with_capacity(func_len as usize);
    for _ in 0..func_len {
      
    }
    Ok(Self {
      major,
      minor,
      funcs
    })
  }
}