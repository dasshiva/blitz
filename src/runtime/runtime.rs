use crate::verifier::*;
use crate::memory::{Memory, ResArea, Stack, i64_to_bytes};

#[derive(Debug, Copy, Clone)]
pub enum CpuStore {
  INT(i64),
  DECIMAL(f64)
}

impl From<CpuStore> for i64 {
  fn from(store: CpuStore) -> Self {
    match store {
      CpuStore::INT(s) => s,
      _ => panic!("Expected an integer value")
    }
  }
}

impl From<CpuStore> for f64 {
  fn from(store: CpuStore) -> Self {
    match store {
      CpuStore::DECIMAL(s) => s,
      _ => panic!("Expected a decimal value")
    }
  }
}

impl From<&Args> for CpuStore {
  fn from(args: &Args) -> Self {
    match args {
      Args::INT(i) => CpuStore::INT(*i),
      Args::DECIMAL(d) => CpuStore::DECIMAL(*d),
      _ => unreachable!()
    }
  }
}

pub struct Runtime {
  cpu: Vec<i64>,
  fcpu: Vec<f64>,
  mem: Memory,
  sp: Stack,
  unit: Unit,
}

impl Runtime {
  pub fn new(unit: Unit) -> Self {
    let mut cpu: Vec<i64> = Vec::new();
    let mut fcpu: Vec<f64> = Vec::new();
    for _ in 0..32 {
      cpu.push(0);
      fcpu.push(0.0);
    }
    let mut mem = match Memory::new(2 * 1024 * 1024) {
        Ok(s) => s,
        Err(e) => panic!("Failed to allocate memory {e}")
    };
    let area = ResArea(String::from("stack"), 0x400, 0x400 + (4 * 1024));
    let sp = Stack::new(&area);
    mem.new_reserved_area(area);
    Self {
      cpu,
      fcpu,
      mem,
      sp,
      unit
    }
  }
  
  pub fn run(&mut self, name: &str, mut pc: usize) {
    let mut index = -1;
    let mut tocall: Option<String> = None;
    self.cpu[31] = unsafe { std::mem::transmute::<usize, i64>(self.sp.top) };
    for i in 0..self.unit.funcs.len() {
      if self.unit.funcs[i].name == name {
        index = i as isize;
        break;
      }
    }
    if index == -1 {
      panic!("{} function not found", name);
    }
    let main = self.unit.funcs.get(index as usize).unwrap(); 
    while pc < main.ins.len() {
      let ins = &main.ins[pc];
      match ins.name {
        0 => {}
        1 => {
          let args = ins.args.as_ref().unwrap();
          match args[0] {
            Args::OFFSET(reg, ..) => {
              let store = unsafe { std::mem::transmute::<i64, usize>(self.cpu[reg as usize]) };
              let arg = i64_to_bytes(self.iget(&ins.sign[1..], &args[1]).into());
              self.mem.write(store, &arg);
              pc += 1;
              continue;
            }
            _ => {}
          }
          let reg = args[0].get_reg_num() as usize;
          let arg = self.iget(&ins.sign[1..], &args[1]);
          self.cpu[reg] = i64::from(arg);
        }
        2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => {
          let args = ins.args.as_ref().unwrap();
          let delta = ins.name - 2;
          let reg = args[0].get_reg_num() as usize;
          let arg1 = i64::from(self.iget(&ins.sign[1..2], &args[1]));
          let arg2 = i64::from(self.iget(&ins.sign[2..3], &args[2]));
          self.cpu[reg] = calc_int(arg1, arg2, delta).into();
        }
        12 => {
          let args = ins.args.as_ref().unwrap();
          pc = match args[0] {
            Args::INT(s) => s as usize,
            _ => panic!("Expected a label to jump to")
          };
          continue;
        }
        13 | 14 | 15 | 16 | 17 | 18 => {
          let args = ins.args.as_ref().unwrap();
          let arg1 = i64::from(self.iget(&ins.sign[0..1], &args[0]));
          let arg2 = i64::from(self.iget(&ins.sign[1..2], &args[1]));
          let mut res = false;
          match ins.name - 13  {
            0 => res = arg1 == arg2,
            1 => res = arg1 != arg2,
            2 => res = arg1 <= arg2,
            3 => res = arg1 < arg2,
            4 => res = arg1 >= arg2,
            5 => res = arg1 > arg2,
            _ => unreachable!()
          }
          if !res {
            pc += 2;
            continue;
          }
        }
        19 => {
          let func = &ins.args.as_ref().unwrap()[0];
          match func {
            Args::STRING(s) => {
              tocall = Some(s.to_string()); 
              break;
            }
            _ => unreachable!()
          }
        }
        20 => {
          let args = ins.args.as_ref().unwrap();
          let reg = args[0].get_reg_num() as usize;
          let arg = self.fget(&ins.sign[1..], &args[1]);
          self.fcpu[reg] = f64::from(arg);
        }
        21 | 22 | 23 | 24 | 25 => {
          let args = ins.args.as_ref().unwrap();
          let delta = ins.name - 21;
          let reg = args[0].get_reg_num() as usize;
          let arg1 = f64::from(self.fget(&ins.sign[1..2], &args[1]));
          let arg2 = f64::from(self.fget(&ins.sign[2..3], &args[2]));
          self.fcpu[reg] = calc_dec(arg1, arg2, delta).into();
        }
        26 | 27 | 28 | 29 | 30 | 31  => {
          let args = ins.args.as_ref().unwrap();
          let arg1 = f64::from(self.fget(&ins.sign[0..1], &args[0]));
          let arg2 = f64::from(self.fget(&ins.sign[1..2], &args[1]));
          let mut res = false;
          match ins.name - 26  {
            0 => res = arg1 == arg2,
            1 => res = arg1 != arg2,
            2 => res = arg1 <= arg2,
            3 => res = arg1 < arg2,
            4 => res = arg1 >= arg2,
            5 => res = arg1 > arg2,
            _ => unreachable!()
          }
          if !res {
            pc += 2;
            continue;
          }
        }
        32 => {
          let args = ins.args.as_ref().unwrap();
          let arg1 = f64::from(self.fget(&ins.sign[0..1], &args[0]));
          self.sp.pushf(arg1, &mut self.mem);
          self.cpu[31] = unsafe { std::mem::transmute::<usize, i64>(self.sp.top) };
        }
        33 => {
          let args = ins.args.as_ref().unwrap();
          let reg = args[0].get_reg_num() as usize;
          self.fcpu[reg] = self.sp.popf(&self.mem);
          self.cpu[31] = unsafe { std::mem::transmute::<usize, i64>(self.sp.top) };
        }
        34 => {
          let args = ins.args.as_ref().unwrap();
          let arg1 = i64::from(self.iget(&ins.sign[0..1], &args[0]));
          self.sp.push(arg1, &mut self.mem);
          self.cpu[31] = unsafe { std::mem::transmute::<usize, i64>(self.sp.top) };
        }
        35 => {
          let args = ins.args.as_ref().unwrap();
          let reg = args[0].get_reg_num() as usize;
          self.cpu[reg] = self.sp.pop(&self.mem);
          self.cpu[31] = unsafe { std::mem::transmute::<usize, i64>(self.sp.top) };
        }
        36 => {
          let args = ins.args.as_ref().unwrap();
          let target = args[0].get_reg_num() as usize;
          match args[1] {
            Args::OFFSET(reg, off) => {
              let store = unsafe { std::mem::transmute::<i64, usize>(self.cpu[reg as usize] + off) };
              self.cpu[target] = unsafe { std::mem::transmute::<usize, i64>(store) };
            }
            _ => panic!("Invalid argument to lea")
          }
        }
        _ => panic!("Invalid or unimplemented opcode {}", ins.name)
      }
      pc += 1;
      println!("r0-r29{:?},\nf0-f29{:?}", self.cpu, self.fcpu);
    }
    if tocall.is_some() {
      self.run(&tocall.unwrap(), 0);
      tocall = None;
      self.run(name, pc + 1);
    }
  }
  
  pub fn iget(&self, sign: &str, arg: &Args) -> CpuStore {
    match sign {
      "I" => CpuStore::from(arg),
      "R" => CpuStore::INT(self.cpu[arg.get_reg_num() as usize]),
      _ => panic!("Cannot get integer value")
    }
  }
  
  pub fn fget(&self, sign: &str, arg: &Args) -> CpuStore {
    match sign {
      "D" => CpuStore::from(arg),
      "R" => CpuStore::DECIMAL(self.fcpu[arg.get_reg_num() as usize]),
      _ => panic!("Cannot get decimal value")
    }
  }
}

fn calc_int(arg1: i64, arg2: i64, mode: u16) -> CpuStore {
  match mode {
    0 => CpuStore::INT(arg1 + arg2),
    1 => CpuStore::INT(arg1 - arg2),
    2 => CpuStore::INT(arg1 * arg2),
    3 => CpuStore::INT(arg1 / arg2),
    4 => CpuStore::INT(arg1 % arg2),
    5 => CpuStore::INT(arg1 | arg2),
    6 => CpuStore::INT(arg1 & arg2),
    7 => CpuStore::INT(arg1 ^ arg2),
    8 => CpuStore::INT(arg1 << arg2),
    9 => CpuStore::INT(arg1 >> arg2),
    _ => unreachable!()
  }
}

fn calc_dec(arg1: f64, arg2: f64, mode: u16) -> CpuStore {
  match mode {
    0 => CpuStore::DECIMAL(arg1 + arg2),
    1 => CpuStore::DECIMAL(arg1 - arg2),
    2 => CpuStore::DECIMAL(arg1 * arg2),
    3 => CpuStore::DECIMAL(arg1 / arg2),
    4 => CpuStore::DECIMAL(arg1 % arg2),
    _ => unreachable!()
  }
}