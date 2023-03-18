use std::env;
use std::panic;

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
  let file = match std::fs::read(&args[1]) {
    Ok(s) => s,
    Err(e) => panic!("Error: {e}")
  }
}