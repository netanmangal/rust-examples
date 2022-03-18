// use std::io;

// fn main() {
//     let mut user_input = String::new();
//     println!("Hi there, please input: ");

//     match io::stdin().read_line(&mut user_input) {
//         Ok(_) => {
//             println!("Your input: {}", user_input);
//         },
//         Err(e) => {
//             println!("Error: {}", e);
//         }
//     }
// }



use std::io;

fn print_table(num: u32) {
    for i in 1..11 {
        println!("{} * {} = {}", num, i, num * i);
    }
}

fn main() {
    let mut number = String::new();
    println!("Enter a number");

    match io::stdin().read_line(&mut number) {
        Ok(_) => {
            print_table(number.trim().parse().expect("Errorrr"))
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}