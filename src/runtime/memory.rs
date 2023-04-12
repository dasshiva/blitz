use crate::exec::Cpu;
use crate::utils;

pub const READ: u8 = 0b001;
pub const WRITE: u8 = 0b010;
pub const EXEC: u8 = 0b100;

impl Cpu {
    pub fn write(&mut self, mut offset: usize, buf: &[u8]) {
        match self.check_permission(offset, offset + buf.len(), WRITE) {
          Ok(..) => {
            for i in buf {
              self.memory[offset] = *i;
              offset += 1;
            }
          }
          Err(e) => {
            self.special[3] = e as usize;
            self.throw(1);
          }
        }
    }

    pub fn check_permission(&self, beg: usize, end: usize, perm: u8) -> Result<(), u8>{
      if (self.regs.get(81) & (1 << 3)) == 0 {
        return Ok(());
      }
      let mut first = 0; 
      let mut second = 0;
      let mut index = 0;
      for area in &self.gdt {
        if area.0 <= beg && area.1 >= beg {
          if (area.2 & perm) == 0 {
            return Err(area.2);
          } 
          first = index;
        }

        if area.0 <= end && area.1 >= end {
          if (area.2 & perm) == 0 {
            return Err(area.2);
          }
          second = index + 1;
          break;
        }
        index += 1;
      }

      for area in &self.gdt[first..second] {
        if (area.2 & perm) == 0 {
          return Err(area.2);
        }
      }
      Ok(())
    }

    pub fn read(&mut self, offset: usize, len: usize) -> &[u8] {
      match self.check_permission(offset, offset + len, READ) {
        Ok(..) => &self.memory[offset..(offset + len)],
        Err(e) => {
          self.special[3] = e as usize;
          self.throw(1);
          unreachable!()
        }
      }
    }
    
    pub fn read_u32(&mut self, from: usize) -> u32 {
      let content = self.read(from, from + 4);
      utils::make_u32(content)
    }
    
    pub fn read_u64(&mut self, from: usize) -> u64 {
      let content = self.read(from, from + 8);
      utils::make_u64(content)
    }
}
