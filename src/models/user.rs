use time::PrimitiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<PrimitiveDateTime>,
}

#[derive(Deserialize, Serialize)]
pub struct RespondUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: Option<PrimitiveDateTime>,
}

#[derive(Deserialize, Serialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}
