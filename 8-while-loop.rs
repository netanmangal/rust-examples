fn main() {
    // table of 10 - 10 iterations

    let multiplier = 10;
    let mut n = 1;           //   <--   Keep it mutable

    while n <=10 {
        println!("{} * {} = {}", multiplier, n, multiplier * n);
        n += 1;    // ++ and -- operators are not supported in rust.
    }
}