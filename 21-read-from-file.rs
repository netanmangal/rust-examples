use std::fs::File;
use std::io::prelude::*;

fn main() {
  let mut file = File::open("info.txt").expect("Can't open file!");
  let mut contents = String::new();

  // file.read_to_string(&mut contents)
  //   .expect("Opps!! Can't read file...");

  match file.read_to_string(&mut contents) {
    Ok(_) => {
      println!("Read from the file...");
    },
    Err(e) => {
      println!("{}", e);
    }
  }

  println!("{}", contents);
}