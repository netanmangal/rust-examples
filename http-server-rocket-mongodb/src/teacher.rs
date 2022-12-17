use rocket::*;

#[get("/?<id>")]
pub fn get_teacher(id: &str) -> String {
    format!("Hello.\nFetching details of teacher - id {}", id)
}