const TOKEN_NAME: &str = "NETIZAQ";
const DECIMALS: u8 = 18;

fn main() {
    println!("We are {}ians", TOKEN_NAME);

    if DECIMALS == 18 {
        println!("{} token will have 18 decimals.", TOKEN_NAME);
    }

    // Error 
    // DECIMALS = 9;  
}