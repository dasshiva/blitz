use crate::memory::Memory;
use crate::utils;

const MAGIC: u32 = 0xAFC;
const MAJOR: u16 = 0x1;
const MINOR: u16 = 0x0;

pub struct Cpu {
  regs: Vec<usize>,
  memory: Memory,
}

#[derive(Debug)]
enum Args {
  INT(u64),
  DECIMAL(f64),
  OFFSET(u8, u64),
  REG(u8)
}

impl Args {
  pub fn get_reg(&self) -> u8 {
    match self {
      Args::REG(reg) => *reg,
      _ => panic!("Not a register!")
    }
  }
  
  pub fn get_int(&self) -> u64 {
    match self {
      Args::INT(val) => *val,
      _ => panic!("Not an integer!")
    }
  }
  
  pub fn get_decimal(&self) -> f64 {
    match self {
      Args::DECIMAL(val) => *val,
      _ => panic!("Not a float!")
    }
  }
  
  pub fn get_off(&self) -> (u8, u64) {
    match self {
      Args::OFFSET(reg, off) => (*reg, *off),
      _ => panic!("Not an integer!")
    }
  }
}
fn read_args(value: u8, code: &[u8], offset: &mut usize) -> Args {
  match value {
    0..=26 => Args::REG(value),
    29 => {
      let num = utils::make_u64(&code[*offset..(*offset + 8)]);
      *offset += 8;
      let reg = (num >> 59) as u8;
      let off = num & ((1 << 59) - 1);
      Args::OFFSET(reg, off)
    }
    30 => {
      let num = utils::make_u64(&code[*offset..(*offset + 8)]);
      *offset += 8;
      Args::INT(num)
    }
    31 => {
      let num = utils::make_u64(&code[*offset..(*offset + 8)]);
      *offset += 8;
      let arg = unsafe { std::mem::transmute::<u64, f64>(num) };
      Args::DECIMAL(arg)
    }
    _ => panic!("Unknown instruction argument {value}")
  }
}

fn decode(ins: u32, code: &[u8], offset: usize) -> (Vec<Args>, usize) {
  let mut pc = offset;
  let mut args_vec: Vec<Args> = Vec::with_capacity(3);
  let arg = (ins & 0xFFFF) as u16;
  let arg1 = (arg >> 11) as u8;
  let arg2 = ((arg >> 6) & 31) as u8;
  let arg3 = ((arg >> 1) & 63) as u8;
  pc += 4;
  args_vec.push(read_args(arg1, code, &mut pc));
  args_vec.push(read_args(arg2, code, &mut pc));
  args_vec.push(read_args(arg3, code, &mut pc));
  println!("{args_vec:?}");
  (args_vec, pc)
}

impl Cpu {
  fn new(memory: Memory) -> Self {
    let mut regs: Vec<usize> = Vec::with_capacity(26);
    for _ in 0..26 {
      regs.push(0);
    }
    Self {
      regs,
      memory
    }
  }
  
  pub fn init(memory: Memory) -> Self {
    let header = memory.read("Code", 0, 8);
    let magic = utils::make_u32(&header[0..4]);
    if magic != MAGIC {
      panic!("Not a blitz executable!");
    }
    let major = utils::make_u16(&header[4..6]);
    let minor = utils::make_u16(&header[6..8]);
    if major > MAJOR || minor > MINOR {
      panic!("Unsupported blitz version {major}.{minor}");
    }
    Cpu::new(memory)
  }
  
  pub fn exec(&mut self) {
    let code = self.memory.read("Code", 0x00000, 0x7DFFF);
    let offset = utils::make_u64(&code[8..16]);
    println!("{offset:X}");
    self.real_exec(offset as usize);
  }
  
  fn real_exec(&mut self, mut pc: usize) {
    let code = self.memory.read("Code", 0x00000, 0x7DFFF);
    loop {
      let ins = utils::make_u32(&code[pc..(pc + 4)]);
      let (args, new) = decode(ins, code, pc);
      match ins >> 16 {
        1 => {
          let reg = args[0].get_reg() as usize;
          self.regs[reg] = args[1].get_int() as usize;
        }
        37 => return,
        _ => panic!("Invalid instruction {}", ins >> 16)
      }
      pc = new;
    }
  }
}