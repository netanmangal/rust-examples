use crate::state::{GENDER, SUBJECTS};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentQueryInput {
    pub name: String,
    pub age: u8,
    pub gender: GENDER,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherQueryInput {
    pub name: String,
    pub age: u8,
    pub gender: GENDER,
    pub subjects: Vec<SUBJECTS>
}