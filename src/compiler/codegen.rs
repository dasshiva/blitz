use crate::parser::*;
use crate::sema::SemUnit;
use std::fs::File;
use std::io::Error;
extern crate file_utils;
use file_utils::write::Write;

fn _write_bytes(file: &mut File, buf: &[u8]) -> Result<(), Error> {
  file.write_u32(buf.len() as u32)?;
  for i in buf {
    file.write_u8(*i)?;
  }
  Ok(())
}

fn _compute_flags_from_attrs(writer: &mut File, attrs: &Option<Vec<Attr>>) -> Result<(), Error> {
  if attrs.is_none() {
    writer.write_u16(0)?;
    return Ok(());
  }
  let mut flag = 0u16;
  for attr in attrs.as_ref().unwrap() {
    match attr.0 {
      Attrs::FIRMWARE => flag |= (1 << 0)
    }
  }
  writer.write_u16(flag)
}

pub fn code_gen(unit: SemUnit) -> Result<(), Error> {
  let mut writer = File::create(unit.name.clone() + ".out")?;
  writer.write_u32(unit.header.magic)?;
  writer.write_u16(unit.header.major)?;
  writer.write_u16(unit.header.minor)?;
  writer.write_usize(unit.header.start)?;
  for func in unit.funcs {
    for ins in func.ins {
      writer.write_u32(ins.opcode)?;
      if ins.size == 4 {
        continue;
      }
      for arg in ins.args {
       if arg.0.is_some() {
         writer.write_i64(arg.0.unwrap())?;
       }
       else {
         writer.write_f64(arg.1.unwrap())?;
       }
      } // arg
    } // ins
  } // func
  Ok(())
}