use std::panic;
use std::env;
mod file;
use file::Handle;
mod r#proc;
mod parser;
mod codegen;
use r#proc::Unit;

fn main() {
   panic::set_hook(Box::new(|panic_info| {
    if let Some(s) = panic_info.payload().downcast_ref::<String>(){
      println!("{s}");
    } 
    else if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
      println!("{s}");
    }
  })); 
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Input file name needed");
  }
  match codegen::code_gen(Unit::new(Handle::new(&args[1]))) {
    Ok(..) => {},
    Err(e) => panic!("{e}")
  }
}
