use crate::state::GENDER;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentQueryInput {
    pub name: String,
    pub age: u8,
    pub gender: GENDER,
}
