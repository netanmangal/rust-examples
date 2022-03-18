extern crate rand;
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1, 101);
    println!("Random number: {}", random_number);
}