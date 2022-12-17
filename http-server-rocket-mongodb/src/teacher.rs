use crate::state::*;
use rocket::*;

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