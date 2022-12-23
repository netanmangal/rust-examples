use crate::query::*;
use crate::state::*;
use rocket::{get, post};
use rocket::serde::json::Json;

#[get("/?<id>")]
pub fn get_student(id: u8) -> String {
    let mut student: &StudentInfo = &StudentInfo::new();

    unsafe {
        for (_, o) in STUDENTS.iter().enumerate() {
            if id == o.id {
                student = o;
            }
        }
    }

    return format!("{:#?}", student);
}

#[get("/count")]
pub fn get_student_count() -> String {
    unsafe {
        return STUDENT_COUNT.to_string();
    }
}

#[post("/add", format = "application/json", data = "<student>")]
pub fn add_student(student: Json<StudentQueryInput>) -> Json<StudentInfo> {
    unsafe {
        let new_student = StudentInfo::create_student(
            STUDENT_COUNT + 1,
            &student.name[..],
            student.age,
            student.gender,
        );

        STUDENTS.push(new_student.clone());

        STUDENT_COUNT += 1;

        return Json(new_student);
    }
}
