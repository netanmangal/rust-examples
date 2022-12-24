#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;

mod student;
mod teacher;
mod state;
mod init;
mod query;

#[get("/")]
fn index() -> String {
    return format!("Hello world. Welcome to index!");
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, init::init_server])
        .mount("/student", routes![student::get_student, student::get_student_count, student::add_student])
        .mount("/teacher", routes![teacher::get_teacher, teacher::get_teacher_count, teacher::add_teacher])
}
