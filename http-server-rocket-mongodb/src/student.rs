use crate::state::*;
use rocket::get;

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
