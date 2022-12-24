use crate::query::*;
use crate::state::*;
use mongodb::bson::doc;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::{get, post, State};

#[get("/?<id>")]
pub async fn get_teacher(id: i32, db: &State<Database>) -> Json<TeacherInfo> {
    let teacher = db
        .collection::<TeacherInfo>("teacher")
        .find_one(doc! { "id": id }, None)
        .await
        .unwrap()
        .expect("Missing teacher with given id.");

    Json(teacher)
}

#[get("/count")]
pub async fn get_teacher_count(db: &State<Database>) -> String {
    let teacher_count = db
        .collection::<TeacherInfo>("teacher")
        .count_documents(None, None)
        .await
        .unwrap();

    teacher_count.to_string()
}

#[post("/add", format = "application/json", data = "<teacher>")]
pub async fn add_teacher(
    teacher: Json<TeacherQueryInput>,
    db: &State<Database>,
) -> Json<TeacherInfo> {
    let teacher_count: u8 = db
        .collection::<TeacherInfo>("teacher")
        .count_documents(None, None)
        .await
        .unwrap()
        .try_into()
        .unwrap();

    let new_teacher = TeacherInfo::create_teacher(
        teacher_count + 1,
        &teacher.name[..],
        teacher.age,
        teacher.gender,
        &teacher.subjects,
    );

    db.collection::<TeacherInfo>("teacher")
        .insert_one(&new_teacher, None)
        .await
        .ok();

    return Json(new_teacher);
}
