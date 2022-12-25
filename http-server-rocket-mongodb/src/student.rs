use crate::query::*;
use crate::state::*;
use mongodb::bson::{self, doc};
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};

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
    let student_count: i32 = db
        .collection::<StudentInfo>("student")
        .count_documents(None, None)
        .await
        .unwrap()
        .try_into()
        .unwrap(); // student_coint is u64. so try_into is used to convert to i32

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

#[put("/update", format = "application/json", data = "<student>")]
pub async fn update_student(student: Json<StudentInfo>, db: &State<Database>) -> Json<StudentInfo> {
    db.collection::<StudentInfo>("student")
        .update_one(
            doc! {
                "id": student.id
            },
            doc! {
                "$set": bson::to_bson( &(student.clone()).into_inner() ).unwrap()
            },
            None,
        )
        .await
        .ok();

    return student;
}

#[delete("/delete", format = "application/json", data = "<id>")]
pub async fn delete_student(id: Json<StudentDeleteQueryInput>, db: &State<Database>) -> String {
    db.collection::<StudentInfo>("student")
        .delete_one(
            doc! {
                "id": id.into_inner().id
            },
            None,
        )
        .await
        .ok();

    format!("Deleted successfully")
}
