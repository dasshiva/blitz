extern crate mmap_rs;
use mmap_rs::{MmapOptions, MmapFlags, MmapMut, Error};

pub struct Arena (usize, usize);

pub struct Memory {
  size: usize,
  used: usize,
  mem: MmapMut
}

impl Memory {
  pub fn new(size: usize) -> Result<Self, Error> {
    Ok(Self {
      size,
      used: 0,
      mem: MmapOptions::new(size)?
           .with_flags(MmapFlags::COPY_ON_WRITE)
           .map_mut()?,
    })
  }
  
  pub fn alloc(&mut self, size: usize) -> Arena {
    let used = self.used + size;
    if used >= self.size {
      panic!("Out of memory");
    }
    let ret = Arena(self.used, used);
    self.used += size;
    ret
  }
  
  pub fn write(&mut self, arena: &Arena, data: &[u8]) {
    if data.len() > arena.1 - arena.0 + 1 {
      panic!("Cannot write data of len {} to block of len {}", 
       data.len(), arena.1 - arena.0 + 1)
    }
    let mut j = 0;
    for i in arena.0..arena.1 {
      self.mem[i] = data[j];
      j += 1;
    }
  }
}
