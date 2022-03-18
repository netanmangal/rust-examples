use std::collections::HashMap;
use std::io;

fn main() {
    let mut balances = HashMap::new();

    // add values
    balances.insert("netan", 50);
    balances.insert("james", 20);
    balances.insert("rajat", 14);

    // find length
    println!("length of hashmap: {}", balances.len());

    // retrieving value
    match balances.get("netan") {
        Some(value) => {
            println!("Value: {}", value);
        },
        None => {
            println!("Opps! key-value snot found.");
        }
    }

    // Looping in hashmap
    for (key, value) in &balances {
        println!("Address: {}, balance: {}", key, value);
    }

    let mut input = String::new();
    println!("Enter the key: ");
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    // contains key
    println!("Do we have the balance for {}?  {}", input, balances.contains_key(&input as &str));
}