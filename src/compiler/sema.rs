use crate::r#proc::Unit;
use crate::parser::Args::*;

const MAGIC: u32 = 0xAFC;
const FIRMWARE_MAGIC: u32 = 0xFAE;
const MAJOR: u16 = 0x1;
const MINOR: u16 = 0x0;

pub struct Header {
  pub magic: u32,
  pub major: u16,
  pub minor: u16,
  pub start: usize
}

type Args = (Option<i64>, Option<f64>);
pub struct Ins {
  ins: u32,
  args: Vec<Args>,
  size: usize
}

pub struct Func {
  ins: Vec<Ins>,
  size: usize
}

pub struct SemUnit {
  pub header: Header,
  pub funcs: Vec<Func>,
}

pub fn sem_analyse(unit: Unit) -> SemUnit {
  let mut header = Header {
    magic: MAGIC,
    major: MAJOR,
    minor: MINOR,
    start: 16
  };
  let mut offset = 16usize;
  let mut funcs: Vec<Func> = Vec::new();
  for func in &unit.funcs {
    let mut f: Vec<Ins> = Vec::new();
    let mut size = 0;
    for ins in &func.ins {
      let mut ins_size = 0;
      let mut opcode = (ins.name as u32) << 16;
      if ins.len == 1 {
        f.push(Ins {
          ins: opcode,
          args: Vec::new(),
          size: 4
        });
        ins_size = 4;
        size += ins_size;
        continue;
      }
      ins_size += 4;
      let mut chunk = 11;
      let mut args_vec: Vec<Args> = Vec::new();
      for args in ins.args.as_ref().unwrap() {
        match args {
          INT(i) => {
            args_vec.push((Some(*i), None));
            ins_size += 8;
            opcode |= (31 << chunk);
            chunk -= 5;
          }
          DECIMAL(d) => {
            args_vec.push((None, Some(*d)));
            ins_size += 8;
            opcode |= (32 << chunk);
            chunk -= 5;
          }
          REGISTER(r) => {
            opcode |= ((*r as u32) << chunk);
            chunk -= 5;
          }
          _ => todo!()
        }
      }
      f.push(Ins {
       ins: opcode,
       args: args_vec,
       size: ins_size
      });
      size += ins_size;
    }
    if func.name == "main" || func.name == "_start" {
      header.start = offset;
    }
    offset += size;
    funcs.push(Func {
      ins: f,
      size
    });
  }
  
  SemUnit {
    header,
    funcs
  }
}