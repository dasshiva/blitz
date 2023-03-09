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
      Args::OFFSET(..) => ret.push('O' as u8),
      _ => unreachable!()
    }
  }
  ret
}

fn write_bytes(file: &mut File, buf: &[u8]) -> Result<(), Error> {
  file.write_u32(buf.len() as u32)?;
  for i in buf {
    file.write_u8(*i)?;
  }
  Ok(())
}

fn compute_flags_from_attrs(writer: &mut File, attrs: &Option<Vec<Attr>>) -> Result<(), Error> {
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

pub fn code_gen(unit: Unit) -> Result<(), Error> {
  let mut writer = File::create(unit.name.clone() + ".out")?;
  writer.write_u32(MAGIC)?;
  writer.write_u16(MAJOR)?;
  writer.write_u16(MINOR)?;
  compute_flags_from_attrs(&mut writer, &unit.attrs)?;
  writer.write_u32(unit.funcs.len() as u32)?;
  for func in unit.funcs {
    let name = func.name.as_bytes();
    write_bytes(&mut writer, &name)?;
    writer.write_u32(func.ins.len() as u32)?;
    for ins in func.ins {
      writer.write_u16(ins.name as u16)?;
      if ins.name.is_no_arg() {
        write_bytes(&mut writer, &['N' as u8])?;
        continue;
      }
      let flags = compute_flags(&ins);
      write_bytes(&mut writer, &flags)?;
      for arg in ins.args.as_ref().unwrap() {
        match arg {
          Args::REGISTER(r) => writer.write_u8(*r)?,
          Args::INT(i) => writer.write_i64(*i)?,
          Args::DECIMAL(d) => writer.write_f64(*d)?,
          Args::STRING(s) => write_bytes(&mut writer, &s.as_bytes())?,
          Args::OFFSET(reg, off) => {
            writer.write_u8(*reg)?;
            writer.write_i64(*off)?;
          }
          _ => unreachable!()
        }
      }
    }
  }
  Ok(())
}