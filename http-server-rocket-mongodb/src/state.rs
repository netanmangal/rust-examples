use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum GENDER {
    MALE,
    FEMALE,
    OTHER,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SUBJECTS {
    SCIENCE,
    MATHS,
    PUNJABI,
    ENGLISH,
    BIOLOGY,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentInfo {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub still_studies: bool,
    pub gender: GENDER,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeacherInfo {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub gender: GENDER,
    pub subjects: Vec<SUBJECTS>,
}

impl StudentInfo {
    pub fn create_student(id: i32, name: &str, age: i32, gender: GENDER) -> StudentInfo {
        StudentInfo {
            id: id,
            name: String::from(name),
            age: age,
            still_studies: true,
            gender: gender,
        }
    }
}

impl TeacherInfo {
    pub fn create_teacher(
        id: i32,
        name: &str,
        age: i32,
        gender: GENDER,
        subjects: &Vec<SUBJECTS>,
    ) -> TeacherInfo {
        TeacherInfo {
            id: id,
            name: name.to_string(),
            age: age,
            gender: gender,
            subjects: subjects.clone(),
        }
    }
}
