fn main() {
    let mut n = 0;

    /*
     *infinite loop
     */
    // loop {
    //     println!("{}", n);
    //     n += 1;
    // }

    // print even numbers between 0..100;
    loop {
        if n > 100 {
            break
        };

        if n % 2 == 0 {
            println!("{}", n);
        }
        
        n += 1;
    }
}