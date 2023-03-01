use std::panic;
mod file;
use file::Handle;
mod r#proc;
mod parser;
use r#proc::Unit;
extern crate serde;
extern crate postcard;

fn main() {
   panic::set_hook(Box::new(|panic_info| {
    if let Some(s) = panic_info.payload().downcast_ref::<String>(){
      println!("{s}");
    } 
    else if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
      println!("{s}");
    }
  }));
  Unit::new(Handle::new("hello.su")).gen();
}
