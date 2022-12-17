use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum GENDER {
    MALE,
    FEMALE,
    OTHER,
}

#[derive(Serialize, Deserialize, Debug)]
enum SUBJECTS {
    SCIENCE,
    MATHS,
    PUNJABI,
    ENGLISH,
    BIOLOGY,
}

#[derive(Serialize, Deserialize, Debug)]
struct StudentInfo {
    id: u8,
    name: String,
    age: u8,
    still_studies: bool,
    gender: GENDER,
}

#[derive(Serialize, Deserialize, Debug)]
struct TeacherInfo {
    id: u8,
    name: String,
    age: u8,
    gender: GENDER,
    subjects: Vec<SUBJECTS>,
}

impl StudentInfo {
    fn createStudent(id: u8, name: &str, age: u8, gender: GENDER) -> StudentInfo {
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
    fn createTeacher(id: u8, name: &str, age: u8, gender: GENDER) -> TeacherInfo {
        TeacherInfo {
            id: id,
            name: name.to_string(),
            age: age,
            gender: gender,
            subjects: vec![SUBJECTS::SCIENCE, SUBJECTS::BIOLOGY],
        }
    }
}
