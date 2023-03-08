extern crate mmap_rs;
use mmap_rs::{MmapOptions, MmapFlags, MmapMut, Error};

#[derive(Clone)]
pub struct ResArea(pub String, pub usize, pub usize);

pub struct Memory {
  size: usize,
  resarea: Vec<ResArea>,
  mem: MmapMut
}

impl Memory {
  pub fn new(size: usize) -> Result<Self, Error> {
    Ok(Self {
      size,
      mem: MmapOptions::new(size)?
           .with_flags(MmapFlags::COPY_ON_WRITE)
           .map_mut()?,
      resarea: Vec::new()
    })
  }
  
  pub fn new_reserved_area(&mut self, area: ResArea) {
    if area.2 > 0x1500 {
      panic!("Reserved area cannot be created over 0x1500")
    }
    self.resarea.push(area);
  }
  
  pub fn write(&mut self, mut offset: usize, data: &[u8]) {
    if data.len() + offset > self.size {
      panic!("Cannot write data of len {} as it will cause OOM", 
       data.len())
    }
    for i in 0..data.len() {
      self.mem[offset] = data[i];
      offset += 1;
    }
  }
  
  pub fn read(&self, offset: usize, len: usize) -> &[u8] {
    if offset + len > self.size {
      panic!("Cannot read {} bytes as it will read over memory limits", len);
    }
    &self.mem[offset..(offset + len + 1)]
  }
}

pub struct Stack {
  top: usize,
  beg: usize,
  end: usize,
}

impl Stack {
  pub fn new(area: &ResArea) -> Self {
    if area.0 != "stack" {
      unreachable!()
    }
    Self {
      top: 0,
      beg: area.1,
      end: area.2
    }
  }
}