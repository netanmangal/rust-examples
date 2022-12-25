#![feature(proc_macro_hygiene, decl_macro)]

mod db;
mod init;
mod query;
mod state;
mod student;
mod teacher;

use db::DB;
use rocket::*;

#[get("/")]
fn index() -> String {
    return format!("Hello world. Welcome to index!");
}

#[launch]
async fn rocket() -> _ {
    let db: DB = DB::connect_db().await.unwrap();

    rocket::build()
        .mount("/", routes![index, init::init_server])
        .mount(
            "/student",
            routes![
                student::get_student,
                student::get_student_count,
                student::add_student,
                student::update_student,
                student::delete_student
            ],
        )
        .mount(
            "/teacher",
            routes![
                teacher::get_teacher,
                teacher::get_teacher_count,
                teacher::add_teacher,
                teacher::update_teacher,
                teacher::delete_teacher
            ],
        )
        .manage(db.client.database("RustMongo"))
}
