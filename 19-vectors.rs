use std::io;
use std::io::Write;

fn print_vector(vec: &Vec<i32>) {
    for i in vec.iter() {
        print!("{}  ", i);
        io::stdout().flush().unwrap();
    }
    println!();
}

fn main() {
    // let vec: Vec<i32> = Vec::new();
    let mut vec: Vec<i32> = vec![1, 2, 3, 4];

    // indexing an element
    println!("{}", vec[2]);

    // pushing element at last
    vec.push(40);
    print_vector(&vec);

    // pop from the last
    vec.pop();
    print_vector(&vec);

    // remove from specific index
    vec.remove(1);
    print_vector(&vec);
}