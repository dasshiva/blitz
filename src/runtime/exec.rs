use std::hint::unreachable_unchecked;

use crate::memory::Memory;
use crate::utils;

const MAGIC: u32 = 0xAFC;
const MAJOR: u16 = 0x1;
const MINOR: u16 = 0x0;

pub struct Cpu {
  regs: Vec<usize>,
  fregs: Vec<f64>,
  memory: Memory,
}

#[derive(Debug)]
enum Args {
  INT(u64),
  DECIMAL(f64),
  OFFSET(u8, u64),
  REG(u8),
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
  let arg3 = ((arg >> 1) & 31) as u8;
  pc += 4;
  args_vec.push(read_args(arg1, code, &mut pc));
  args_vec.push(read_args(arg2, code, &mut pc));
  args_vec.push(read_args(arg3, code, &mut pc));
  (args_vec, pc)
}

impl Cpu {
  fn new(memory: Memory) -> Self {
    let mut regs: Vec<usize> = Vec::with_capacity(27);
    let mut fregs: Vec<f64> = Vec::with_capacity(27);
    for _ in 0..27 {
      regs.push(0);
      fregs.push(0.0);
    }
    Self {
      regs,
      fregs,
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
    let sp = self.memory.get_area("Stack");
    self.regs[25] = sp.2;
    self.real_exec(offset as usize);
  }
  
  #[allow(mutable_transmutes)]
  fn real_exec(&mut self, mut pc: usize) {
    let code = self.memory.read("Code", 0x00000, 0x7DFFF);
    let mut depth: Vec<usize> = Vec::new();
    loop {
      let ins = utils::make_u32(&code[pc..(pc + 4)]);
      let opcode = ins >> 16;
      let (args, new) = decode(ins, code, pc);
      match opcode {
        0 => {},
        1 => {
          let arg = match &args[1] {
            Args::REG(r) => self.regs[*r as usize],
            Args::INT(s) => *s as usize,
            Args::OFFSET(reg, off) => {
              let address = self.regs[*reg as usize] + *off as usize;
              utils::make_u64(self.memory.raw_read(address, address + 8)) as usize
            }
            _ => unreachable!()
          };
          match &args[0] {
            Args::REG(r) => self.regs[*r as usize] = arg,
            Args::OFFSET(reg, off) => {
              let address = self.regs[*reg as usize] + *off as usize;
              let content = utils::u64_to_u8(arg as u64);
              let mem = unsafe {
               std::mem::transmute::<&Memory, &mut Memory>(&self.memory)
             };
              mem.raw_write(address, address + 8, &content);
            }
            _ => unreachable!()
          }
        }
        2..=11 => {
          let reg = args[0].get_reg() as usize;
          let arg1 = match &args[1] {
            Args::INT(s) => *s as usize,
            Args::REG(r) => self.regs[*r as usize],
            _ => unreachable!()
          };
          let arg2 = match &args[2] {
            Args::INT(s) => *s as usize,
            Args::REG(r) => self.regs[*r as usize],
            _ => unreachable!()
          };
          match opcode - 2 {
            0 => self.regs[reg] = arg1 + arg2,
            1 => self.regs[reg] = arg1 - arg2,
            2 => self.regs[reg] = arg1 * arg2,
            3 => self.regs[reg] = arg1 / arg2,
            4 => self.regs[reg] = arg1 % arg2,
            5 => self.regs[reg] = arg1 | arg2,
            6 => self.regs[reg] = arg1 & arg2,
            7 => self.regs[reg] = arg1 ^ arg2,
            8 => self.regs[reg] = arg1 << arg2,
            9 => self.regs[reg] = arg1 >> arg2,
            _ => unreachable!()
          }
        }
        12 | 19 => {
          let offset = args[0].get_int() as usize;
          depth.push(new);
          pc = offset;
          continue;
        }
        13..=18 => {
          let offset = args[0].get_int() as usize;
          let mut res = false;
          match opcode - 13 {
            0 => res = (self.regs[26] & (1 << 0)) != 0,
            1 => res = (self.regs[26] & (1 << 0)) == 0,
            2 => res = (self.regs[26] & (1 << 1)) != 0 || (self.regs[26] & (1 << 0)) != 0,
            3 => res = (self.regs[26] & (1 << 1)) != 0,
            4 => res = (self.regs[26] & (1 << 2)) != 0 || (self.regs[26] & (1 << 0)) != 0,
            5 => res = self.regs[26] & (1 << 2) != 0,
            _ => unsafe { unreachable_unchecked() }
          }
          if res {
            pc = offset;
            continue;
          }
        }
        20 => {
          let reg = args[0].get_reg() as usize;
          self.fregs[reg] = args[1].get_decimal();
        }
        21..=25 => {
          let reg = args[0].get_reg() as usize;
          let arg1 = match &args[1] {
            Args::DECIMAL(s) => *s,
            Args::REG(r) => self.fregs[*r as usize],
            _ => unreachable!()
          };
          let arg2 = match &args[2] {
            Args::DECIMAL(s) => *s,
            Args::REG(r) => self.fregs[*r as usize],
            _ => unreachable!()
          };
          match opcode - 21 {
            0 => self.fregs[reg] = arg1 + arg2,
            1 => self.fregs[reg] = arg1 - arg2,
            2 => self.fregs[reg] = arg1 * arg2,
            3 => self.fregs[reg] = arg1 / arg2,
            4 => self.fregs[reg] = arg1 % arg2,
            _ => unreachable!()
          }
        }
        26 => {
          match &args[0] {
            Args::REG(r) => self.regs[*r as usize] += 1,
            Args::OFFSET(reg, off) => {
              let address = self.regs[*reg as usize] + *off as usize;
              let arg = utils::make_u64(self.memory.raw_read(address, address + 8)) + 1;
              let content = utils::u64_to_u8(arg as u64);
              let mem = unsafe {
               std::mem::transmute::<&Memory, &mut Memory>(&self.memory)
             };
              mem.raw_write(address, address + 8, &content);
            }
            _ => unreachable!()
          }
        }
        27 => {
          match &args[0] {
            Args::REG(r) => self.regs[*r as usize] -= 1,
            Args::OFFSET(reg, off) => {
              let address = self.regs[*reg as usize] + *off as usize;
              let arg = utils::make_u64(self.memory.raw_read(address, address + 8)) - 1;
              let content = utils::u64_to_u8(arg as u64);
              let mem = unsafe {
               std::mem::transmute::<&Memory, &mut Memory>(&self.memory)
             };
              mem.raw_write(address, address + 8, &content);
            }
            _ => unreachable!()
          }
        }
        28 => {
          match &args[0] {
            Args::REG(r) => self.fregs[*r as usize] += 1.0,
            _ => unreachable!()
          }
        }
        29 => {
          match &args[0] {
            Args::REG(r) => self.fregs[*r as usize] -= 1.0,
            _ => unreachable!()
          }
        }
        30 => {
          let reg = args[0].get_reg() as usize;
          let bit = args[1].get_int() as usize;
          self.regs[reg] |= 1 << bit;
        }
        31 => {
          let reg = args[0].get_reg() as usize;
          let bit = args[1].get_int() as usize;
          self.regs[reg] &= !(1 << bit);
        }
        32 => {
          let arg1 = match &args[0] {
            Args::DECIMAL(s) => *s,
            Args::REG(r) => self.fregs[*r as usize],
            _ => unreachable!()
          };
          let num = utils::u64_to_u8(arg1 as u64);
          self.regs[25] -= 8;
          let mem = unsafe {
            std::mem::transmute::<&Memory, &mut Memory>(&self.memory)
          };
          mem.write("Stack", self.regs[25], &num);
        }
        33 => {
          let reg = args[0].get_reg() as usize;
          let word = self.memory.read("Stack", self.regs[25], 8);
          self.regs[25] += 8;
          self.fregs[reg] = unsafe { std::mem::transmute::<u64, f64>(utils::make_u64(&word)) };
        }
        34 => {
          let arg1 = match &args[0] {
            Args::INT(s) => *s as usize,
            Args::REG(r) => self.regs[*r as usize],
            _ => unreachable!()
          };
          let num = utils::u64_to_u8(arg1 as u64);
          self.regs[25] -= 8;
          let mem = unsafe {
            std::mem::transmute::<&Memory, &mut Memory>(&self.memory)
          };
          mem.write("Stack", self.regs[25], &num);
        }
        35 => {
          let reg = args[0].get_reg() as usize;
          let word = self.memory.read("Stack", self.regs[25], 8);
          self.regs[25] += 8;
          self.regs[reg] = utils::make_u64(&word) as usize;
        }
        36 => {
          let reg = args[0].get_reg() as usize;
          let (target, offset) = args[1].get_off();
          self.regs[reg] = self.regs[target as usize] + offset as usize;
        }
        37 => {
          if depth.len() == 0 {
            return;
          }
          pc = depth.pop().unwrap();
          continue;
        }
        38 => {
          let arg1 = match &args[0] {
            Args::INT(s) => *s as usize,
            Args::REG(r) => self.regs[*r as usize],
            _ => unreachable!()
          };
          let arg2 = match &args[1] {
            Args::INT(s) => *s as usize,
            Args::REG(r) => self.regs[*r as usize],
            _ => unreachable!()
          };
          if arg1 == arg2 {
            self.regs[26] |= 1 << 0;
          } else if arg1 > arg2 {
            self.regs[26] |= 1 << 1;
          } else if arg1 < arg2 {
            self.regs[26] |= 1 << 2;
          }
        }
        39 => {
          let arg1 = match &args[0] {
            Args::DECIMAL(s) => *s,
            Args::REG(r) => self.fregs[*r as usize],
            _ => unreachable!()
          };
          let arg2 = match &args[1] {
            Args::DECIMAL(s) => *s,
            Args::REG(r) => self.fregs[*r as usize],
            _ => unreachable!()
          };
          if arg1 == arg2 {
            self.regs[26] |= 1 << 0;
          } else if arg1 > arg2 {
            self.regs[26] |= 1 << 1;
          } else if arg1 < arg2 {
            self.regs[26] |= 1 << 2;
          }
        }
        _ => panic!("Invalid instruction {}", ins >> 16)
      }
      pc = new;
      println!("{:?}, {pc}", self.regs);
    }
  }
}