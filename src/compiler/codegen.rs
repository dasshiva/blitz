use crate::parser::*;
use crate::r#proc::Unit;
use std::fs::File;
use std::io::Error;
extern crate file_utils;
use file_utils::write::Write;

const MAGIC: u32 = 0xAFC;
const MAJOR: u16 = 0x1;
const MINOR: u16 = 0x0;

fn compute_flags(ins: &Instr) -> Vec<u8> {
  let mut ret: Vec<u8> = Vec::new();
  for arg in ins.args.as_ref().unwrap() {
    match arg {
      Args::REGISTER(..) => ret.push('R' as u8),
      Args::INT(..) => ret.push('I' as u8),
      Args::DECIMAL(..) => ret.push('D' as u8),
      Args::STRING(..) => ret.push('S' as u8),
    }
  }
  ret
}

fn write_bytes(file: &mut File, buf: &[u8]) -> Result<(), Error> {
  for i in buf {
    file.write_u8(*i)?;
  }
  Ok(())
}

pub fn code_gen(unit: Unit) -> Result<(), Error> {
  let mut writer = File::create(unit.name.clone() + ".out")?;
  writer.write_u32(MAGIC)?;
  writer.write_u16(MAJOR)?;
  writer.write_u16(MINOR)?;
  writer.write_u32(unit.funcs.len() as u32)?;
  for func in unit.funcs {
    let name = func.name.as_bytes();
    write_bytes(&mut writer, &name)?;
    writer.write_u32(func.ins.len() as u32)?;
    for ins in func.ins {
      writer.write_u16(ins.name as u16)?;
      let flags = compute_flags(&ins);
      writer.write_u8(flags.len() as u8)?;
      if flags.len() != 0 {
        write_bytes(&mut writer, &flags)?;
      }
      for arg in ins.args.as_ref().unwrap() {
        match arg {
          Args::REGISTER(r) => writer.write_u8(*r)?,
          Args::INT(i) => writer.write_i64(*i)?,
          Args::DECIMAL(d) => writer.write_f64(*d)?,
          _ => panic!("Strings are not supported as instruction arguments yet.")
        }
      }
    }
  }
  Ok(())
}