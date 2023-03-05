use crate::verifier::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum CpuStore {
  INT(i64),
  DECIMAL(f64)
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
  cpu: Vec<CpuStore>,
  unit: Unit,
}

impl Runtime {
  pub fn new(unit: Unit) -> Self {
    let mut cpu: Vec<CpuStore> = Vec::new();
    for _ in 0..30 {
      cpu.push(CpuStore::INT(0));
    }
    Self {
      cpu,
      unit
    }
  }
  
  pub fn run(&mut self, name: &str, mut pc: usize) {
    let mut index = -1;
    let mut tocall: Option<String> = None;
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
                CpuStore::DECIMAL(d) => self.cpu[reg] = calc_dec(s as f64, d, delta)
              }
            }
            CpuStore::DECIMAL(d) => {
              match arg2 {
                CpuStore::INT(s1) => self.cpu[reg] = calc_int(d as i64, s1, delta),
                CpuStore::DECIMAL(d1) => self.cpu[reg] = calc_dec(d, d1, delta),
              }
            }
          }
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
          let arg1 = self.get(&ins.sign[0..1], &args[0]);
          let arg2 = self.get(&ins.sign[1..2], &args[1]);
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
        _ => panic!("Invalid or unimplemented opcode {}", ins.name)
      }
      pc += 1;
      println!("{:?}", self.cpu);
    }
    if tocall.is_some() {
      self.run(&tocall.unwrap(), 0);
      self.run(name, pc + 1);
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