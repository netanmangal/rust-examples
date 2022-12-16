#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;

#[get("/")]
fn index() -> String {
    return format!("Hello world. Welcome to index! again");
}

#[get("/<name>/<age>")]
fn hello(name: &str, age: Option<u8>) -> String {
    format!("Hello. Age is {}, name is {}", age.unwrap_or(0), name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/hello", routes![hello])
}