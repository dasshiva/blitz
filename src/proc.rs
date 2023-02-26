use crate::file::Handle;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
  static ref INS: Regex = Regex::new(r"(\w+)").unwrap();
  static ref FUNC_START: Regex = Regex::new(r"func (\w+)").unwrap();
}

pub struct Unit {
  
}

impl Unit {
  pub fn new(src: Handle) -> Unit {
    for line in src {
      
      for cap in FUNC_START.captures_iter(&line) {
        println!("{}", &cap[1]);
      }
      /*for cap in INS.captures_iter(&line) {
        println!("{}", &cap[1]);
      }*/
    }
    Unit {
      
    }
  }
}