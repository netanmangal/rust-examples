fn main() {
    let variable_name = "i am immutable"; // this is immutable variable.
    println!("{}", variable_name);

    // error
    // variable_name = "i am being updated"; 

    /*
    To make variables mutable, we need to use the keyword "mut".
    */
    let mut mutable_variable = "this is the value of mutable variable before updating";
    println!("Before updating variable: {}", mutable_variable);
    mutable_variable = "this is the value of mutable variable after updating";
    println!("After updating variable: {}", mutable_variable);
}