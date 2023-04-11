extern crate mmap_rs;
use std::hint::unreachable_unchecked;

use crate::exec::Cpu;
static SIZE: usize = 2 * 1024 * 1024;

pub const READ: usize = 0b001;
pub const WRITE: usize = 0b010;
pub const EXEC: usize = 0b100;

impl Cpu {
  pub fn write(&mut self, area: &str, mut offset: usize, buf: &[u8]) {
    for i in &self.areas {
      if i.0 == area {
         if i.1 > offset || i.2 < offset {
           panic!("Offset to write is beyond region {area}'s limits");
         }
         if i.2 - i.1 + 1 < buf.len() {
           panic!("Writing beyond the limits of region {area}");
         }
         if (i.3 & WRITE) == 0 {
          panic!("Region {area} does not have write access");
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

         if i.3 & READ == 0 {
           panic!("Region {area} does not have read access");
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
    
    unsafe { unreachable_unchecked() }
  }
  
  pub fn find_area(&self, start: usize, end: usize) -> &ResArea {
    for area in &self.areas {
      if area.1 <= start && area.2 >= end {
        return area;
      }
    }

    unsafe { unreachable_unchecked() }
  }

  pub fn find_permission(&self, addr: usize) -> usize {
    for area in &self.areas {
      if area.1 <= addr && area.2 >= addr {
        return area.3;
      }
    }

    unsafe { unreachable_unchecked() }
  }

  pub fn raw_write(&mut self, mut beg: usize, end: usize, buf: &[u8]) {
    if end >= self.size {
      panic!("Writing beyond memory limits is not allowed");
    }
    let area = self.find_area(beg, end);
    if area.3 & WRITE == 0 {
      panic!("Writing to region with no write access");
    }
    for unit in buf {
      self.mem[beg] = *unit;
      beg += 1;
    }
  }
  
  pub fn raw_read(&self, beg: usize, end: usize) -> &[u8] {
    if end >= self.size {
      panic!("Writing beyond memory limits is not allowed");
    }
    let area = self.find_area(beg, end);
    if area.3 & READ == 0 {
      panic!("Reading from region with no read access");
    }
    &self.mem[beg..end]
  }
}