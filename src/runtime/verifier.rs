extern crate file_utils;
use std::io::Error;
use std::fs::File;
use file_utils::read::Read;

pub struct Unit {
  major: u16,
  minor: u16
}

impl Unit {
  pub fn new(name: &str) -> Result<Self, Error> {
    let mut file = File::open(name)?;
    let magic = file.read_u32()?;
    if magic != 0xAFC {
      panic!("File {name} is not a blitz executable");
    }
    Ok(Self {
      major: file.read_u16()?,
      minor: file.read_u16()?
    })
  }
}