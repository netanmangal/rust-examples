use crate::state::{GENDER, SUBJECTS};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentQueryInput {
    pub name: String,
    pub age: i32,
    pub gender: GENDER,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherQueryInput {
    pub name: String,
    pub age: i32,
    pub gender: GENDER,
    pub subjects: Vec<SUBJECTS>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherDeleteQueryInput {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentDeleteQueryInput {
    pub id: i32,
}
