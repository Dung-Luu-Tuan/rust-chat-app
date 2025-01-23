use serde::{Serialize, Deserialize};
use uuid::Uuid;
use time::PrimitiveDateTime;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub id: Uuid,
    pub room_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: Option<PrimitiveDateTime>,
}

#[derive(Deserialize)]
pub struct NewMessage {
    pub user_id: Uuid,
    pub content: String,
}
