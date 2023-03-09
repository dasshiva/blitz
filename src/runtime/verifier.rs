extern crate file_utils;
use std::io::Error;
use std::fs::File;
use file_utils::read::Read;

fn str_from_bytes(file: &mut File) -> Result<String, Error> {
  let size = file.read_u32()?;
  if size == 0 {
    return Ok(String::new());
  }
  let mut buf: Vec<u8> = Vec::with_capacity(size as usize);
  for _ in 0..size {
    buf.push(file.read_u8()?);
  }
  unsafe {
    Ok(String::from_utf8_unchecked(buf))
  }
}

pub enum Args {
  INT(i64),
  DECIMAL(f64),
  REGISTER(u8),
  STRING(String),
  OFFSET(u8, i64)
}

impl Args {
  pub fn get_reg_num(&self) -> u8 {
    match self {
      Args::REGISTER(s) => *s,
      _ => unreachable!()
    }
  }
}

pub struct Ins {
  pub name: u16,
  pub sign: String,
  pub args: Option<Vec<Args>>
}

impl Ins {
  pub fn new(file: &mut File) -> Result<Self, Error> {
    let name = file.read_u16()?;
    let sign = str_from_bytes(file)?;
    if sign.len() == 0 {
      Ok(Self {
        name,
        sign,
        args: None
      })
    }
    else {
      let mut args: Vec<Args> = Vec::new();
      for byte in sign.as_bytes() {
        match *byte as char {
          'R' => {
            let reg = file.read_u8()?;
            args.push(Args::REGISTER(reg));
          }
          'I' => {
            let int = file.read_i64()?;
            args.push(Args::INT(int));
          }
          'D' => {
            let decimal = file.read_f64()?;
            args.push(Args::DECIMAL(decimal));
          }
          'S' => {
            let string = str_from_bytes(file)?;
            args.push(Args::STRING(string));
          }
          'N' => {
            return Ok(Self {
              name,
              sign,
              args: None
            });
          }
          'O' => {
            let reg = file.read_u8()?;
            let off = file.read_i64()?;
            args.push(Args::OFFSET(reg, off));
          }
          _ => unreachable!()
        }
      }
      Ok(Self {
        name,
        sign,
        args: Some(args)
      })
    }
  }
}

pub struct Function {
  pub name: String,
  pub ins: Vec<Ins>
}

impl Function {
  pub fn new(file: &mut File) -> Result<Self, Error> {
    let name = str_from_bytes(file)?;
    let ins_size = file.read_u32()?;
    let mut ins: Vec<Ins> = Vec::new();
    for _ in 0..ins_size {
      ins.push(Ins::new(file)?);
    }
    Ok(Self {
      name,
      ins
    })
  }
}

pub struct Unit {
  pub major: u16,
  pub minor: u16,
  pub flags: u16,
  pub funcs: Vec<Function>
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
    if major != MAJOR || minor != MINOR {
      panic!("Unsupported blitz version {major}.{minor}");
    }
    let flags = file.read_u16()?;
    let func_len = file.read_u32()?;
    let mut funcs: Vec<Function> = Vec::with_capacity(func_len as usize);
    for _ in 0..func_len {
      funcs.push(Function::new(&mut file)?);
    }
    Ok(Self {
      major,
      minor,
      flags,
      funcs
    })
  }
}