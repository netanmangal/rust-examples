use crate::query::*;
use crate::state::*;
use mongodb::bson::{self, doc};
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};

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
    let teacher_count: i32 = db
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

#[put("/update", format = "application/json", data = "<teacher>")]
pub async fn update_teacher(teacher: Json<TeacherInfo>, db: &State<Database>) -> Json<TeacherInfo> {
    db.collection::<TeacherInfo>("teacher")
        .update_one(
            doc! {
                "id": teacher.id
            },
            doc! {
                "$set": bson::to_bson( &(teacher.clone()).into_inner() ).unwrap()
            },
            None,
        )
        .await
        .ok();

    return teacher;
}

#[delete("/delete", format = "application/json", data = "<id>")]
pub async fn delete_teacher(id: Json<TeacherDeleteQueryInput>, db: &State<Database>) -> String {
    db.collection::<TeacherInfo>("teacher")
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
