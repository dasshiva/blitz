extern crate mmap_rs;
use mmap_rs::{MmapOptions, MmapFlags, MmapMut, Error};

pub fn i64_to_bytes(value: i64) -> [u8; 8] {
  let mut arr = [0; 8];
  let val = unsafe { std::mem::transmute::<i64, u64>(value) };
  arr[0] = (val >> 56) as u8;
  arr[1] = (val >> 48) as u8;
  arr[2] = (val >> 40) as u8;
  arr[3] = (val >> 32) as u8;
  arr[4] = (val >> 24) as u8;
  arr[5] = (val >> 16) as u8;
  arr[6] = (val >> 8) as u8;
  arr[7] = (val >> 0) as u8;
  arr
}

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
    for i in &self.resarea {
      if i.1 >= area.1 && i.2 <= area.2 {
        panic!("Reserved area {} and {} overlap with each other", i.0, area.0);
      }
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
  pub top: usize,
  beg: usize,
  end: usize,
}

impl Stack {
  pub fn new(area: &ResArea) -> Self {
    if area.0 != "stack" {
      unreachable!()
    }
    Self {
      top: area.1,
      beg: area.1,
      end: area.2
    }
  }
  
  pub fn push(&mut self, value: i64, mem: &mut Memory) {
    if self.top + 8 > self.end {
      panic!("Stack overflow");
    }
    let arr = i64_to_bytes(value);
    mem.write(self.top, &arr);
    self.top += 8;
  }
  
  pub fn pop(&mut self, mem: &Memory) -> i64 {
    if self.top - 8 < self.beg {
      panic!("Stack underflow");
    }
    self.top -= 8;
    let array = mem.read(self.top, 8);
    let mut copy = [0u8; 8];
    for i in 0..8 {
      copy[i] = array[i];
    }
    i64::from_be_bytes(copy)
  }
  
  pub fn pushf(&mut self, val: f64, mem: &mut Memory) {
    let value = unsafe { std::mem::transmute::<f64, i64>(val) };
    self.push(value, mem);
  }
  
  pub fn popf(&mut self, mem: &Memory) -> f64 {
    let value = self.pop(mem);
    unsafe {
      std::mem::transmute::<i64, f64>(value)
    }
  }
}