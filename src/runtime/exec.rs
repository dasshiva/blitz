use crate::memory::Memory;
use crate::utils;

const MAGIC: u32 = 0xAFC;
const MAJOR: u16 = 0x1;
const MINOR: u16 = 0x0;

pub struct Cpu {
  regs: Vec<usize>,
  memory: Memory,
}

impl Cpu {
  fn new(mut memory: Memory) -> Self {
    let mut regs: Vec<usize> = Vec::with_capacity(26);
    for i in 0..26 {
      regs.push(0);
    }
    Self {
      regs,
      memory
    }
  }
  
  pub fn init(mut memory: Memory) -> Self {
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
    let mut pc = offset as usize; 
    loop {
      let ins = utils::make_u32(&code[pc..(pc + 4)]);
      match ins >> 16 {
        1 => 
      }
      break;
    }
  }
}