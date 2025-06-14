use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub hashed_password: String,
    pub username: String,
    pub email: String,
}