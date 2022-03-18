use std::fs::File;
use std::io::prelude::*;

fn main() {
  let mut file = File::create("output.txt").expect("Could not create a file!!!");

  file.write_all(b"Welcome to NETIZAQ, Netizans most active qommunity. Where qommunity starts with Q and you.").expect("Cannot write to the file...");

  file = File::open("output.txt").expect("Cannot open file to read...");
  
  let mut contents = String::new();

  match file.read_to_string(&mut contents) {
    Ok(_) => {
      println!("{}", contents);
    },
    Err(e) => {
      println!("{}", e);
    }
  }
}