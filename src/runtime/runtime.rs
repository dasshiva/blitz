use crate::verifier::*;

#[derive(Debug, Copy, Clone)]
pub enum CpuStore {
  INT(i64),
  DECIMAL(f64),
  EMPTY
}

impl From<&Args> for CpuStore {
  fn from(args: &Args) -> Self {
    match args {
      Args::INT(i) => CpuStore::INT(*i),
      Args::DECIMAL(d) => CpuStore::DECIMAL(*d),
      _ => CpuStore::EMPTY
    }
  }
}

pub struct Runtime {
  cpu: Vec<CpuStore>,
  unit: Unit,
  pc: usize
}

impl Runtime {
  pub fn new(unit: Unit) -> Self {
    let mut cpu: Vec<CpuStore> = Vec::new();
    for _ in 0..30 {
      cpu.push(CpuStore::EMPTY);
    }
    Self {
      cpu,
      unit,
      pc: 0
    }
  }
  
  pub fn run(&mut self, name: &str) {
    let mut index = -1;
    for i in 0..self.unit.funcs.len() {
      if self.unit.funcs[i].name == name {
        index = i as isize;
        break;
      }
    }
    if index == -1 {
      panic!("Main function not found");
    }
    let main = self.unit.funcs.get(index as usize).unwrap(); 
    while self.pc < main.ins.len() {
      let ins = &main.ins[self.pc];
      println!("{:?}", self.cpu);
      match ins.name {
        1 => {
          let args = ins.args.as_ref().unwrap();
          let reg = args[0].get_reg_num() as usize;
          let arg = self.get(&ins.sign[1..], &args[1]);
          self.cpu[reg] = arg;
        }
        2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => {
          let args = ins.args.as_ref().unwrap();
          let delta = ins.name - 2;
          let reg = args[0].get_reg_num() as usize;
          let arg1 = self.get(&ins.sign[1..2], &args[1]);
          let arg2 = self.get(&ins.sign[2..3], &args[2]);
          match arg1 {
            CpuStore::INT(s) => {
              match arg2 {
                CpuStore::INT(s1) => self.cpu[reg] = calc_int(s, s1, delta),
                CpuStore::DECIMAL(d) => self.cpu[reg] = calc_dec(s as f64, d, delta),
                CpuStore::EMPTY => self.cpu[reg] = arg1
              }
            }
            CpuStore::DECIMAL(d) => {
              match arg2 {
                CpuStore::INT(s1) => self.cpu[reg] = calc_int(d as i64, s1, delta),
                CpuStore::DECIMAL(d1) => self.cpu[reg] = calc_dec(d, d1, delta),
                CpuStore::EMPTY => self.cpu[reg] = arg1
              }
            }
            CpuStore::EMPTY => {
              match arg2 {
                CpuStore::EMPTY => self.cpu[reg] = arg1,
                _ => self.cpu[reg] = arg2
              }
            }
          }
        }
        _ => unimplemented!()
      }
      self.pc += 1;
      println!("{:?}", self.cpu);
    }
  }
  
  pub fn get(&self, sign: &str, arg: &Args) -> CpuStore {
    match sign {
      "I" | "D" => CpuStore::from(arg),
      "R" => self.cpu[arg.get_reg_num() as usize],
      _ => unreachable!()
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
    5..=9 => panic!("Bitwise operators are not allowed on decimal operands"),
    _ => unreachable!()
  }
}