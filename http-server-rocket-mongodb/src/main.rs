#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;

mod student;
mod teacher;

#[get("/")]
fn index() -> String {
    return format!("Hello world. Welcome to index!");
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/student", routes![student::get_student])
        .mount("/teacher", routes![teacher::get_teacher])
}
