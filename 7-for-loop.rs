fn main() {
    // 1..11 <-- Non inclusive range
    for i in 1..11 {
        println!("{}", i);
    }

    let numbers = 100..111;
    for i in numbers {
        println!("{}", i);
    }

    let fruits = vec!["Apple", "Mango", "Pineapple"];
    
    // for fruit in fruits {      <--    This will work but then you wouldn't be able to use `fruits` later.
    for fruit in fruits.iter() {
        println!("The fruit is - {}", fruit);
    }

    // println!("{}", fruits[0]);

    for (i, fruit) in fruits.iter().enumerate() {
        println!("The fruit at index - {} - is - {}", i, fruit);
    }
}