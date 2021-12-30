fn main() {
    let mut n = 0;

    // print even numbers between 0..100;
    loop {
        n += 1;                //   <-   move this upwards because
        if n > 100 {
            break;
        };

        if n % 2 != 0 {
            continue;
        }
        
        println!("{}", n);
    }
}