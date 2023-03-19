pub struct Limits {
  pub code_start: usize,
  pub code_end: usize,
  pub data_start: usize,
  pub data_end: usize,
  pub stack_start: usize,
  pub stack_end: usize,
  pub heap_start: usize,
  pub heap_end: usize
}

impl Limits {
  pub fn new(size: usize) -> Self {
    let code_start = 0;
    let code_end = (40 * size) / 100;
    let data_start = code_end + 1;
    let data_end = data_start + (10 * size) / 100;
    let stack_start = data_end + 1;
    let stack_end = stack_start + (20 * size) / 100;
    let heap_start = stack_end + 1;
    let heap_end = size;
    Self {
      code_start,
      code_end,
      data_start,
      data_end,
      stack_start,
      stack_end,
      heap_start,
      heap_end
    }
  }
}

pub struct Memory {
  mem: Vec<u8>,
  size: usize,
  limits: Limits
}

impl Memory {
  
}