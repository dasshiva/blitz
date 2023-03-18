use crate::memory::*;
extern crate file_utils;
use std::io::Error;
use std::fs::File;
use file_utils::read::Read;

const MAGIC: u32 = 0xAFC;
const MAJOR: u16 = 0x1;
const MINOR: u16 = 0x0;

pub fn load(name: &str, mem: &mut Memory) -> Result<(), Error> {
  let mut file = File::open(name)?;
  let magic = file.read_u32()?;
  if magic != MAGIC {
    panic!("File {name} is not a blitz executable");
  }
  let major = file.read_u16()?;
  let minor = file.read_u16()?;
  if major != MAJOR || minor != MINOR {
    panic!("Unsupported blitz version {major}.{minor}");
  }
  //let resarea = ResArea("Code", 0x1000, 
  Ok(())
}