use interpreter::{run_code, run_file, run_prompt};
use std::{env, fs::File};

fn main() {
  let args: Vec<String> = env::args().collect();
  let file_name = args.get(1);
    match file_name {
        Some(file_name) => {
            println!("Opening file...");
            run_file(file_name);
        }
        _ => run_prompt(),
    }
}