use rocket::*;

#[get("/?<id>")]
pub fn get_student(id: &str) -> String {
    format!("Hello.\nFetching details of student - id {}", id)
}