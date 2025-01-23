use uuid::Uuid;
use serde::{Serialize, Deserialize};
use time::PrimitiveDateTime;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Room {
    pub id: Uuid,
    pub user1_id: Uuid,
    pub user2_id: Uuid,
    pub name: String,
    pub created_at: Option<PrimitiveDateTime>,
}
