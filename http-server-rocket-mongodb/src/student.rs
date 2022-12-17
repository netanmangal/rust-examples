use crate::queryinput::*;
use crate::state::*;
use rocket::{get, post};

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