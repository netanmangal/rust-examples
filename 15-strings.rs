fn main() {
    let mut string = String::from("Hi there");
    println!("{}", string);
    println!("{}", string.len());

    // append string
    let s2 = ". Welcome to Netan's rust tutorial";
    string.push_str(s2);
    println!("{}", string);

    // travesing
    for (i, c) in string.chars().enumerate() {
        println!("{} -> {}", i, c);
    }

    // splitting with whitespaces as delimiter
    for token in string.split_whitespace() {
        println!("{}", token);
    }
}