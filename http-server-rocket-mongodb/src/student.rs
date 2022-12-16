use rocket::*;

#[get("/<name>/<age>")]
pub fn hello(name: &str, age: Option<u8>) -> String {
    format!("Hello. Age is {}, name is {}", age.unwrap_or(0), name)
}