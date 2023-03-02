use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Handle {
  pub file: String,
  line: usize,
  contents: BufReader<File>
}

impl Handle {
  pub fn new(name: &str) -> Self {
    let h = match File::open(name) {
      Ok(s) => s,
      Err(..) => panic!("Could not open file {}", name)
    };
    Self {
      file: name.to_owned(),
      line : 0,
      contents: BufReader::new(h)
    }
  }
  
  pub fn read_line(&mut self) -> String {
    let mut ret = String::new();
    match self.contents.read_line(&mut ret) {
      Ok(s) => { 
        if s == 0 { 
          return "EOF".to_owned();
        }
        self.line += 1;
        ret.trim().to_string()
      }
      Err(..) => panic!("Failed to read from file {} ", self.file)
    }
  }
  
  pub fn error(&self, msg: &str) -> ! {
    panic!("In File {} at line {} : {}", self.file, self.line, msg)
  }
}
