use crate::parser::*;
use crate::r#proc::Unit;
use std::fs::File;
use std::io::Error;
use bitstream_io::{BigEndian, ByteWriter, ByteWrite};

const MAGIC: u32 = 0xAFC;
const MAJOR: u16 = 0x1;
const MINOR: u16 = 0x0;

fn compute_flags(ins: &Instr) -> u8 {
  if ins.len == 1 {
    return 0;
  }
  let mut flag = 0u16;
  let mut bit = 0;
  for arg in ins.args.as_ref().unwrap() {
    match arg {
      Args::REGISTER(..) => flag |= 1 << bit,
      _ => {}
    }
    bit += 1;
  }
  flag
}

pub fn code_gen(unit: Unit) -> Result<(), Error> {
  let file = File::create(unit.name.clone() + ".out")?;
  let mut writer = ByteWriter::endian(file, BigEndian);
  writer.write(MAGIC)?;
  writer.write(MAJOR)?;
  writer.write(MINOR)?;
  writer.write(unit.funcs.len() as u32)?;
  for func in unit.funcs {
    let name = func.name.as_bytes();
    writer.write_bytes(&name)?;
    for ins in func.ins {
      writer.write(ins.name as u16)?;
      writer.write(compute_flags(&ins))?;
      
    }
  }
  Ok(())
}