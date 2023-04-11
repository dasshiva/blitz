pub(crate) use std::hint::unreachable_unchecked;
use crate::memory::{Memory, EXEC};
use mmap_rs::{MmapMut, MmapOptions, MmapFlags};
use crate::utils;

const MAGIC: u32 = 0xAFC;
const MAJOR: u16 = 0x1;
const MINOR: u16 = 0x0;

#[derive(Debug)]
pub struct Regs(pub [usize; 22]);

impl Regs {
  pub fn new() -> Self {
    Self([0usize; 22])
  }
}

impl Regs {
  fn get(&self, idx: usize) -> usize {
    match idx {
      0..=19 => self.0[idx] & 0xFF, 
      20..=39 => self.0[idx - 20] & 0xFFFF,
      40..=59 => self.0[idx - 40] & 0xFFFFFFFF,
      60..=81 => self.0[idx - 60],
      _ => unsafe { unreachable_unchecked() }
    }
  }
  
  fn set(&mut self, idx: usize, val: usize) {
    match idx {
      0..=19 => self.0[idx] = (self.0[idx] & 0xFFFFFFFFFFFFFF00) | val,
      20..=39 => self.0[idx - 20]  = (self.0[idx] & !0xFFFF) | val,
      40..=59 => self.0[idx - 40]  = (self.0[idx] & !0x00000000ffffffff) | val,
      60..=81 => self.0[idx - 60] = val,
      _ => unsafe { unreachable_unchecked() }
    }
  }
}

#[derive(Debug)]
enum Args {
  INT(u64),
  DECIMAL(f64),
  OFFSET(u8, u64),
  FLAG(bool),
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

  pub fn get_flag(&self) -> bool {
    match self {
      Args::FLAG(r) => *r,
      _ => panic!("Not a flag!")
    }
  }
}

fn read_args(value: u8, code: &[u8], offset: &mut usize) -> Args {
  match value {
    0..=80 => Args::REG(value),
    83 => {
      let num = utils::make_u64(&code[*offset..(*offset + 8)]);
      *offset += 8;
      let reg = (num >> 57) as u8;
      let off = num & ((1 << 57) - 1);
      Args::OFFSET(reg, off)
    }
    81 => {
      let num = utils::make_u64(&code[*offset..(*offset + 8)]);
      *offset += 8;
      Args::INT(num)
    }
    82 => {
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
  let mut args_vec: Vec<Args> = Vec::with_capacity(4);
  let arg1 = ((ins >> 15) & 127) as u8;
  let arg2 = ((ins >> 8) & 127) as u8;
  let arg3 = ((ins >> 1) & 127) as u8;
  pc += 4;
  args_vec.push(read_args(arg1, code, &mut pc));
  args_vec.push(read_args(arg2, code, &mut pc));
  args_vec.push(read_args(arg3, code, &mut pc));
  args_vec.push(Args::FLAG(ins & (1 << 0) != 0));
  (args_vec, pc)
}

pub struct Cpu {
  regs: Regs,
  fregs: [f64; 20],
  special: [usize; 6],
  memory: MmapMut
}

impl Cpu {
  fn new(mem: usize) -> Self {
    Self {
      regs: Regs::new(),
      fregs: [0.0f64; 20],
      special: [0usize; 6],
      memory: match MmapOptions::new(mem).unwrap().with_flags(MmapFlags::COPY_ON_WRITE).map_mut() {
        Ok(s) => s,
        Err(e) => panic!("Error allocating memory {e}")
      }
    }
  }
  
  pub fn init() -> Self {
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
    Cpu::new()
  }
  
  pub fn exec(&mut self) {
    let code = self.memory.read("Code", 0x00000, 0x7DFFF);
    let offset = utils::make_u64(&code[8..16]);
    let sp = self.memory.get_area("Stack");
    self.regs.set(80, sp.2);
    self.real_exec(offset as usize);
  }
  
  #[allow(mutable_transmutes)]
  fn real_exec(&mut self, mut pc: usize) {
    let code = self.memory.read("Code", 0x00000, 0x7DFFF);
    let mut depth: Vec<usize> = Vec::new();
    loop {
      let ins = utils::make_u32(&code[pc..(pc + 4)]);
      let opcode = ins >> 22;
      let (args, new) = decode(ins, code, pc);
      match opcode {
        0 => {},
        1 => {
          let arg = match &args[1] {
            Args::REG(r) => self.regs.get(*r as usize),
            Args::INT(s) => *s as usize,
            Args::OFFSET(reg, off) => {
              let address = self.regs.get(*reg as usize) + *off as usize;
              utils::make_u64(self.memory.raw_read(address, address + 8)) as usize
            }
            _ => unreachable!()
          };
          match &args[0] {
            Args::REG(r) => self.regs.set(*r as usize, arg),
            Args::OFFSET(reg, off) => {
              let address = self.regs.get(*reg as usize) + *off as usize;
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
            Args::REG(r) => self.regs.get(*r as usize),
            _ => unreachable!()
          };
          let arg2 = match &args[2] {
            Args::INT(s) => *s as usize,
            Args::REG(r) => self.regs.get(*r as usize),
            _ => unreachable!()
          };
          match opcode - 2 {
            0 => self.regs.set(reg, arg1 + arg2),
            1 => self.regs.set(reg, arg1 - arg2),
            2 => self.regs.set(reg, arg1 * arg2),
            3 => {
              if arg2 == 0 {
                self.throw(0, pc);
              }
              self.regs.set(reg, arg1 / arg2)
            }
            4 => {
              if arg2 == 0 {
                self.throw(0, pc);
              }
              self.regs.set(reg, arg1 % arg2)
            }
            5 => self.regs.set(reg, arg1 | arg2),
            6 => self.regs.set(reg, arg1 & arg2),
            7 => self.regs.set(reg, arg1 ^ arg2),
            8 => self.regs.set(reg, arg1 << arg2),
            9 => self.regs.set(reg, arg1 >> arg2),
            _ => unsafe {
                unreachable_unchecked()
            }
          }
        }
        12 => {
          let arg = match &args[0] {
            Args::REG(r) => self.regs.get(*r as usize),
            Args::INT(s) => *s as usize,
            _ => unsafe {
                unreachable_unchecked()
            }
          };
          let perm = self.memory.find_permission(arg);
          if perm & EXEC == 0 {
            self.special[3] = arg;
            self.throw(1, pc);
          }
          pc = arg;
          continue;
        }
        13..=18 => {
          let offset = args[0].get_int() as usize;
          let res;
          match opcode - 13 {
            0 => res = (self.regs.get(81) & (1 << 0)) != 0,
            1 => res = (self.regs.get(81) & (1 << 0)) == 0,
            2 => res = (self.regs.get(81) & (1 << 1)) != 0 || (self.regs.get(81) & (1 << 0)) != 0,
            3 => res = (self.regs.get(81) & (1 << 1)) != 0,
            4 => res = (self.regs.get(81) & (1 << 2)) != 0 || (self.regs.get(81) & (1 << 0)) != 0,
            5 => res = self.regs.get(81) & (1 << 2) != 0,
            _ => unsafe { unreachable_unchecked() }
          }
          if res {
            pc = offset;
            continue;
          }
        }
        19 => {
          let offset = args[0].get_int() as usize;
          depth.push(new);
          pc = offset;
          continue;
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
            3 => {
              if arg2 == 0.0f64 {
                self.throw(0, pc);
              }
              self.fregs[reg] = arg1 / arg2;
            }
            4 => {
              if arg2 == 0.0f64 {
                self.throw(0, pc);
              }
              self.fregs[reg] = arg1 / arg2;
            }
            _ => unreachable!()
          }
        }
        26 => {
          match &args[0] {
            Args::REG(r) => self.regs.set(*r as usize, self.regs.get(*r as usize) + 1),
            Args::OFFSET(reg, off) => {
              let address = self.regs.get(*reg as usize) + *off as usize;
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
            Args::REG(r) => self.regs.set(*r as usize, self.regs.get(*r as usize) - 1),
            Args::OFFSET(reg, off) => {
              let address = self.regs.get(*reg as usize) + *off as usize;
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
          self.regs.set(reg, self.regs.get(reg) | 1 << bit);
        }
        31 => {
          let reg = args[0].get_reg() as usize;
          let bit = args[1].get_int() as usize;
          self.regs.set(reg, self.regs.get(reg) & !(1 << bit));
        }
        32 => {
          let arg1 = match &args[0] {
            Args::DECIMAL(s) => *s,
            Args::REG(r) => self.fregs[*r as usize],
            _ => unreachable!()
          };
          let num = utils::u64_to_u8(arg1 as u64);
          self.regs.set(80, self.regs.get(80) - 8);
          let mem = unsafe {
            std::mem::transmute::<&Memory, &mut Memory>(&self.memory)
          };
          mem.write("Stack", self.regs.get(80), &num);
        }
        33 => {
          let reg = args[0].get_reg() as usize;
          let word = self.memory.read("Stack", self.regs.get(80), 8);
          self.regs.set(80, self.regs.get(80) + 8);
          self.fregs[reg] = unsafe { std::mem::transmute::<u64, f64>(utils::make_u64(&word)) };
        }
        34 => {
          let arg1 = match &args[0] {
            Args::INT(s) => *s as usize,
            Args::REG(r) => self.regs.get(*r as usize),
            _ => unreachable!()
          };
          let num = utils::u64_to_u8(arg1 as u64);
          self.regs.set(80, self.regs.get(80) - 8);
          let mem = unsafe {
            std::mem::transmute::<&Memory, &mut Memory>(&self.memory)
          };
          mem.write("Stack", self.regs.get(80), &num);
        }
        35 => {
          let reg = args[0].get_reg() as usize;
          let word = self.memory.read("Stack", self.regs.get(80), 8);
          self.regs.set(80, self.regs.get(80) + 8);
          self.regs.set(reg, utils::make_u64(&word) as usize);
        }
        36 => {
          let reg = args[0].get_reg() as usize;
          let (target, offset) = args[1].get_off();
          self.regs.set(reg, self.regs.get(target as usize) + offset as usize);
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
            Args::REG(r) => self.regs.get(*r as usize),
            _ => unreachable!()
          };
          let arg2 = match &args[1] {
            Args::INT(s) => *s as usize,
            Args::REG(r) => self.regs.get(*r as usize),
            _ => unreachable!()
          };
          if arg1 == arg2 {
            self.regs.set(81, self.regs.get(81) | 1 << 0);
          } else if arg1 > arg2 {
            self.regs.set(81, self.regs.get(81) | 1 << 1);
          } else if arg1 < arg2 {
             self.regs.set(81, self.regs.get(81) | 1 << 2);
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
            self.regs.set(81, self.regs.get(81) | 1 << 0);
          } else if arg1 > arg2 {
            self.regs.set(81, self.regs.get(81) | 1 << 1);
          } else if arg1 < arg2 {
             self.regs.set(81, self.regs.get(81) | 1 << 2);
          }
        }
        40 => {
          if args[3].get_flag() {
            match opcode - 40 {
              0 => self.special[0] = args[0].get_int() as usize,
              _ => unsafe {
                  unreachable_unchecked()
              }
            }
          }
          else {
            self.throw(3, pc);
          }
        }
        50 => {
          let ty = args[0].get_int();
          if ty == 0 && !args[3].get_flag() {
            panic!("Attempt to execute privileged instruction with privilege bit off")
          }
          match ty {
            0 => {
              match self.special[1] {
                0 => panic!("Attempt to divide by zero at pc = {}", self.special[2]),
                1 => panic!("Attempt to execute from memory region marked not executable at pc = {}", self.special[2]),
                2 => panic!("Illegal opcode {} at pc = {}", self.special[3], self.special[2]),
                3 => panic!("Attempt to execute privileged instruction with privilege bit off at pc = {}", self.special[2]),
                _ => unreachable!()
              }
            }
           _ => unreachable!()
         }
       }
      _ => {
        self.special[3] = (opcode >> 22) as usize;
        self.throw(2, pc);
      }
    }
      pc = new;
      println!("{:?}, {pc}", self.regs);
    }
  }

  #[allow(mutable_transmutes)]
  fn throw(&self, extype: usize, pc: usize) {
    let inst = unsafe { std::mem::transmute::<&Cpu, &mut Cpu>(self) };
    inst.special[1] = extype;
    inst.special[2] = pc;
    inst.real_exec(self.special[0]);
  }
}