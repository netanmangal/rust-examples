pub fn print_table (num: u32) {
    for i in 1..11 {
        println!("{} * {} = {}", num, i, num * i);
    }
}