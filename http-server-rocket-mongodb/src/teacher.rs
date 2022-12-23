use crate::query::*;
use crate::state::*;
use rocket::{serde::json::Json, *};

#[get("/?<id>")]
pub fn get_teacher(id: u8) -> String {
    let mut teacher: &TeacherInfo = &TeacherInfo::new();

    unsafe {
        for (_, o) in TEACHERS.iter().enumerate() {
            if id == o.id {
                teacher = o;
            }
        }
    }

    format!("{:#?}", teacher)
}

#[get("/count")]
pub fn get_teacher_count() -> String {
    unsafe {
        return TEACHER_COUNT.to_string();
    }
}

#[post("/add", format = "application/json", data = "<teacher>")]
pub fn add_teacher(teacher: Json<TeacherQueryInput>) -> Json<TeacherInfo> {
    unsafe {
        let new_teacher = TeacherInfo::create_teacher(
            TEACHER_COUNT + 1,
            &teacher.name[..],
            teacher.age,
            teacher.gender,
            &teacher.subjects
        );

        TEACHERS.push(new_teacher.clone());
        TEACHER_COUNT += 1;

        return Json(new_teacher);
    }
}
