extern crate mmap_rs;
use mmap_rs::{MmapMut, MmapOptions, MmapFlags};
static SIZE: usize = 2 * 1024 * 1024;

pub struct ResArea(pub String, pub usize, pub usize);
pub struct Memory {
  mem: MmapMut,
  size: usize,
  areas: Vec<ResArea>
}

impl Memory {
  pub fn new(size: usize) -> Self {
   let mem = match MmapOptions::new(size).unwrap().with_flags(MmapFlags::COPY_ON_WRITE).map_mut() {
      Ok(s) => s,
      Err(e) => panic!("Error allocating memory {e}")
   };
    Self {
      mem,
      size,
      areas: Vec::new()
    }
  }
  
  fn new_area(&mut self, name: &str, start: usize, end: usize) {
    let area = ResArea(name.to_string(), start, end);
    self.areas.push(area);
  }
  
  pub fn write(&mut self, area: &str, mut offset: usize, buf: &[u8]) {
    for i in &self.areas {
      if i.0 == area {
         if i.1 > offset || i.2 < offset {
           panic!("Offset to write is beyond region {area}'s limits");
         }
         if i.2 - i.1 + 1 < buf.len() {
           panic!("Writing beyond the limits of region {area}");
         }
         for i in buf {
           self.mem[offset] = *i;
           offset += 1;
         }
         return;
      }
    }
    
    panic!("Memory region {area} not found");
  }
  
  pub fn read(&self, area: &str, offset: usize, len: usize) -> &[u8] {
    for i in &self.areas {
      if i.0 == area {
         if i.1 > offset || i.2 < offset {
           panic!("Offset to read is beyond region {area}'s limits");
         }
         if i.2 - i.1 + 1 < len {
           panic!("Reading beyond the limits of region {area}");
         }
         return &self.mem[offset..(offset + len)];
      }
    }
    
    panic!("Memory region {area} not found")
  }
  
  pub fn get_area(&self, name: &str) -> &ResArea {
    for area in &self.areas {
      if area.0 == name {
        return area;
      }
    }
    panic!("Area {name} not found")
  }
  
  pub fn init(code: &[u8]) -> Self {
    let mut mem = Memory::new(SIZE);
    mem.new_area("Code", 0x00000, 0x7DFFF);
    mem.write("Code", 0x00000, code);
    mem.new_area("Data", 0x7E000, 0xFDFFF);
    mem.new_area("Stack", 0xFE000, 0xFFFFF);
    mem.new_area("Heap", 0xFF000, SIZE - 1);
    mem
  }
}