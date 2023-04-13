use crate::sema::SemUnit;
use std::fs::File;
use std::io::Error;
extern crate file_utils;
use crate::parser::*;
use crate::r#proc::Token;
use file_utils::write::Write;

fn write_bytes(file: &mut File, buf: &[u8]) -> Result<(), Error> {
  for i in buf {
    file.write_u8(*i)?;
  }
  Ok(())
}

pub fn code_gen(unit: SemUnit) -> Result<(), Error> {
  let mut writer = File::create(unit.name.clone() + ".out")?;
  writer.write_u32(unit.header.magic)?;
  writer.write_u16(unit.header.major)?;
  writer.write_u16(unit.header.minor)?;
  writer.write_usize(unit.header.start)?;
  writer.write_usize(unit.header.data)?;
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

 for data in unit.data {
    match data.2 {
      Token::INT(s) => {
        match data.0 {
          Type::BYTE => writer.write_u8(s as u8)?,
          Type::SHORT => writer.write_u16(s as u16)?,
          Type::INT => writer.write_u32(s as u32)?,
          Type::LONG => writer.write_u64(s as u64)?,
          _ => unreachable!()
        }
      }
      Token::STRING(mut s) => {
        s += "\0";
        write_bytes(&mut writer, &s.as_bytes())?
      }
      _ => unreachable!()
    }
  }
  Ok(())
}