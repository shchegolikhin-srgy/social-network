use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User{
    hashed_password: String,
    username: String,
    email: String,
}