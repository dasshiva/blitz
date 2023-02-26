use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Handle {
  file: String,
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
      line : 1,
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

impl Iterator for Handle {
  type Item = String;
  
  fn next(&mut self) -> Option<Self::Item> {
    let line = self.read_line();
    match line.as_ref() {
      "EOF" => None,
       _ => Some(line)
    }
  }
}