#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;

mod student;

#[get("/")]
fn index() -> String {
    return format!("Hello world. Welcome to index! again");
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/student", routes![student::hello])
}