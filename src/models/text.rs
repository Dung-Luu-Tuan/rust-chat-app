use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct TextItem {
    pub id: Uuid,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewText {
    pub text: String,
}
