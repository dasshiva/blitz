use std::fs;
use std::env;
use std::panic;
mod lexer;
use lexer::Lexer;

fn main() {
  let args: Vec<String> = env::args().collect();
 /* for (arg in args) {
    match(&arg) {
      "-
    }
  } */
  panic::set_hook(Box::new(|panic_info|{ 
    if let Some(s) = panic_info.payload().downcast_ref::<String>(){
      println!("{s}"); 
    }
    else if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
      println!("{s}"); 
    }
  }));
  
  if args.len() < 2 {
    panic!("Need a filename");
  }
  let file = fs::read_to_string(&args[1]).expect("File not found");
  let lexer = Lexer::new(file.as_bytes());
}