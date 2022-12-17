use rocket::post;

use crate::state::*;

#[post("/init")]
pub fn init_server() -> String {
    // we are using unsafe block because modifying static variables
    // it's because - multiple threads can use static variable simultaneously which is unsafe.
    unsafe {
        STUDENTS.push(StudentInfo::create_student(1, "netan", 30, GENDER::MALE));
        STUDENTS.push(StudentInfo::create_student(2, "grace", 25, GENDER::FEMALE));

        TEACHERS.push(TeacherInfo::create_teacher(1, "akash", 45, GENDER::MALE));
        TEACHERS.push(TeacherInfo::create_teacher(2, "preeti", 53, GENDER::FEMALE));
    }

    format!("Server init - Success")
}
