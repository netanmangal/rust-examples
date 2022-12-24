use crate::query::*;
use crate::state::*;
use mongodb::bson::doc;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::{get, post, State};

#[get("/?<id>")]
pub async fn get_student(id: i32, db: &State<Database>) -> Json<StudentInfo> {
    let r = db
        .collection::<StudentInfo>("student")
        .find_one(doc! { "id": id }, None)
        .await
        .unwrap()
        .expect("Missing student with given id.");

    return Json(r);
}

#[get("/count")]
pub async fn get_student_count(db: &State<Database>) -> String {
    let count: u64 = db
        .collection::<StudentInfo>("student")
        .count_documents(None, None)
        .await
        .unwrap();

    count.to_string()
}

#[post("/add", format = "application/json", data = "<student>")]
pub async fn add_student(
    student: Json<StudentQueryInput>,
    db: &State<Database>,
) -> Json<StudentInfo> {
    let student_count: u8 = db
        .collection::<StudentInfo>("student")
        .count_documents(None, None)
        .await
        .unwrap()
        .try_into()
        .unwrap(); // student_coint is u64. so try_into is used to convert to u8

    let new_student = StudentInfo::create_student(
        student_count + 1,
        &student.name[..],
        student.age,
        student.gender,
    );

    db.collection::<StudentInfo>("student")
        .insert_one(&new_student, None)
        .await
        .ok();

    return Json(new_student);
}
