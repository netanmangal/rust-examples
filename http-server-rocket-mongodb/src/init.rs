use crate::state::*;
use mongodb::Database;
use rocket::{post, State};

#[post("/init")]
pub async fn init_server(db: &State<Database>) -> Option<String> {
    let new_students: Vec<StudentInfo> = vec![
        StudentInfo::create_student(1, "netan", 30, GENDER::MALE),
        StudentInfo::create_student(2, "grace", 25, GENDER::FEMALE),
    ];

    db.collection("student")
        .insert_many(new_students, None)
        .await
        .ok();

    db.collection("teacher")
        .insert_many(
            [
                TeacherInfo::create_teacher(
                    1,
                    "akash",
                    45,
                    GENDER::MALE,
                    &vec![SUBJECTS::SCIENCE, SUBJECTS::BIOLOGY],
                ),
                TeacherInfo::create_teacher(
                    2,
                    "preeti",
                    53,
                    GENDER::FEMALE,
                    &vec![SUBJECTS::SCIENCE, SUBJECTS::BIOLOGY],
                ),
            ],
            None,
        )
        .await
        .ok();

    Some(format!(
        r"
        Server init - Success. 
        2 students and teacher records inserted."
    ))
}
