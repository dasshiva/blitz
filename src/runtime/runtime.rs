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
  
  pub fn run(&mut self) {
    let mut index = -1;
    for i in 0..self.unit.funcs.len() {
      if self.unit.funcs[i].name == "main" {
        index = i as isize;
        break;
      }
    }
    if index == -1 {
      panic!("Main function not found");
    }
    let main = self.unit.funcs.get(index as usize).unwrap(); 
    for ins in &main.ins {
      println!("{:?}", self.cpu);
      match ins.name {
        1 => {
          let args = ins.args.as_ref().unwrap();
          let reg = args[0].get_reg_num() as usize;
          let arg = self.get(&ins.sign[1..], &args[1]);
          self.cpu[reg] = arg;
        }
        _ => unimplemented!()
      }
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