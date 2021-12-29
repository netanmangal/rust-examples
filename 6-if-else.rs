fn main() {
    let rohan_likes = "Apple";

    // if (rohan_likes == "Apple")   <--  Warning #[warn(unused_parens)]
    if (rohan_likes == "Apple") {
        println!("Give him Apple!");
    } else if rohan_likes == "Mango" {
        println!("Give him Mango!");
    } else {
        println!("Rohan neither wanta Apple nor Mango.");
    }
}