use crate::r#proc::{Unit, Token};
use crate::parser::{Args::*, Attrs, Type, Data};

const MAGIC: u32 = 0xAFC;
const MAJOR: u16 = 0x1;
const MINOR: u16 = 0x0;

pub struct Header {
  pub magic: u32,
  pub major: u16,
  pub minor: u16,
  pub start: usize,
  pub data: usize
}

type Args = (Option<i64>, Option<f64>);
pub struct Ins {
  pub opcode: u32,
  pub args: Vec<Args>,
  pub size: usize
}

pub struct Func {
  pub ins: Vec<Ins>,
  pub size: usize,
}

pub struct SemUnit {
  pub name: String,
  pub header: Header,
  pub funcs: Vec<Func>,
  pub data: Vec<Data>
}

pub fn sem_analyse(unit: Unit) -> SemUnit {
  let mut header = Header {
    magic: MAGIC,
    major: MAJOR,
    minor: MINOR,
    start: 15,
    data: 0
  };
  let mut offset = 24usize;
  let mut offset_table: Vec<(usize, String)> = Vec::new();
  if unit.data.len() != 0 {
    let mut start = 0;
    for define in &unit.define {
      if define.0 == "DATA_BEGIN" {
        match define.1 {
          Token::INT(s) => start = s as usize,
          _ => unreachable!()
        }
        break;
      }
    }
    for var in &unit.data {
      offset_table.push((start, var.1.clone()));
      match var.0 {
        Type::BYTE => start += 1,
        Type::SHORT => start += 2,
        Type::INT => start += 4,
        Type::LONG => start += 8,
        Type::STRING => {
          match &var.2 {
            Token::STRING(s) => start += s.len(),
            _ => unreachable!()
          }
        }
      }
    }
  }
  let mut funcs: Vec<Func> = Vec::new();
  for func in &unit.funcs {
    offset_table.push((offset, func.name.clone()));
    let mut f: Vec<Ins> = Vec::new();
    let mut size = 0;
    for ins in &func.ins {
      let mut ins_size = 0;
      let mut opcode = (ins.name as u32) << 22;
      if ins.len == 1 {
        f.push(Ins {
          opcode,
          args: Vec::new(),
          size: 4
        });
        ins_size = 4;
        size += ins_size;
        continue;
      }
      ins_size += 4;
      let mut chunk = 15;
      let mut args_vec: Vec<Args> = Vec::new();
      for args in ins.args.as_ref().unwrap() {
        match args {
          INT(i) => {
            args_vec.push((Some(*i), None));
            ins_size += 8;
            opcode |= (81 & 0x7F) << chunk;
            chunk -= 7;
          }
          DECIMAL(d) => {
            args_vec.push((None, Some(*d)));
            ins_size += 8;
            opcode |= (82 & 0x7F) << chunk;
            chunk -= 7;
          }
          REGISTER(r) => {
            opcode |= ((*r as u32) & 0x7F) << chunk;
            chunk -= 7;
          }
          OFFSET(reg, off) => {
            let arg: u64 = ((*reg as u64) << 57) | (*off as u64);
            let actual= unsafe { std::mem::transmute::<u64, i64>(arg) };
            ins_size += 8;
            opcode |= (83 & 0x7F) << chunk;
            chunk -= 7;
            args_vec.push((Some(actual), None));
          }
          STRING(s) => {
            let mut found = false;
            for i in &offset_table {
              if &i.1 == s {
                let off = unsafe { std::mem::transmute::<usize, i64>(i.0) };
                args_vec.push((Some(off), None));
                ins_size += 8;
                opcode |= (81 & 0x7F) << chunk;
                chunk -= 7;
                found = true;
                break;
              }
            }
            
            if !found {
              panic!("Function or label {s} not found");
            }
          } 
        }
      }
      f.push(Ins {
       opcode,
       args: args_vec,
       size: ins_size
      });
      size += ins_size;
    }
    if func.name == "_start" {
      header.start = offset;
    }
    offset += size;
    for attr in func.attrs.as_ref().unwrap() {
      for ins in &mut f {
        match attr.0 {
          Attrs::FIRMWARE => ins.opcode |= 1 << 0
        }
      }
    }
    funcs.push(Func {
      ins: f,
      size,
    });
  }
  header.data = offset;
  
  SemUnit {
    name: unit.name,
    header,
    funcs,
    data: unit.data
  }
}
