use std::env;
mod tokeniser;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Filename!")
  }
  let file = &args[1];
}