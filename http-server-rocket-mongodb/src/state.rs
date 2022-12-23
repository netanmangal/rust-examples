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
    pub id: u8,
    pub name: String,
    pub age: u8,
    pub still_studies: bool,
    pub gender: GENDER,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeacherInfo {
    pub id: u8,
    pub name: String,
    pub age: u8,
    pub gender: GENDER,
    pub subjects: Vec<SUBJECTS>,
}

impl StudentInfo {
    pub fn new() -> StudentInfo {
        StudentInfo {
            id: 0,
            name: String::new(),
            age: 0,
            still_studies: false,
            gender: GENDER::OTHER,
        }
    }

    pub fn create_student(id: u8, name: &str, age: u8, gender: GENDER) -> StudentInfo {
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
    pub fn new() -> TeacherInfo {
        TeacherInfo {
            id: 0,
            name: String::new(),
            age: 0,
            gender: GENDER::OTHER,
            subjects: Vec::new(),
        }
    }

    pub fn create_teacher(id: u8, name: &str, age: u8, gender: GENDER, subjects: &Vec<SUBJECTS>) -> TeacherInfo {
        TeacherInfo {
            id: id,
            name: name.to_string(),
            age: age,
            gender: gender,
            subjects: subjects.clone(),
        }
    }
}

pub static mut STUDENTS: Vec<StudentInfo> = Vec::new();
pub static mut TEACHERS: Vec<TeacherInfo> = Vec::new();
pub static mut STUDENT_COUNT: u8 = 0;
pub static mut TEACHER_COUNT: u8 = 0;