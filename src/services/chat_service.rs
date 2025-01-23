use crate::models::{message::{Message, NewMessage}, room::Room};
use sqlx::PgPool;
use uuid::Uuid;

/// Tạo hoặc lấy phòng chat giữa 2 người dùng
pub async fn get_or_create_room(
    pool: &PgPool,
    user1_id: Uuid,
    user2_id: Uuid,
) -> Result<Room, sqlx::Error> {
    // Tìm phòng giữa 2 người dùng
    let existing_room = sqlx::query_as!(
        Room,
        r#"
        SELECT id, user1_id, user2_id, name, created_at
        FROM rooms
        WHERE (user1_id = $1 AND user2_id = $2)
           OR (user1_id = $2 AND user2_id = $1)
        "#,
        user1_id,
        user2_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(room) = existing_room {
        // Phòng đã tồn tại
        Ok(room)
    } else {
        // Tạo phòng mới
        let room_name = format!("{}-{}", user1_id, user2_id); // Tên phòng là ID của 2 user
        let new_room = sqlx::query_as!(
            Room,
            r#"
            INSERT INTO rooms (user1_id, user2_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, user1_id, user2_id, name, created_at
            "#,
            user1_id,
            user2_id,
            room_name
        )
        .fetch_one(pool)
        .await?;

        Ok(new_room)
    }
}

/// Lấy tất cả tin nhắn trong phòng
pub async fn fetch_messages(
    pool: &PgPool,
    room_id: &Uuid,
) -> Result<Vec<Message>, sqlx::Error> {
    sqlx::query_as!(
        Message,
        r#"
        SELECT id, room_id, user_id, content, created_at
        FROM messages
        WHERE room_id = $1
        ORDER BY created_at ASC
        "#,
        room_id
    )
    .fetch_all(pool)
    .await
}

pub async fn save_message(
    pool: &sqlx::PgPool,
    message: &NewMessage,
    room_id: Uuid,
) -> Result<Message, sqlx::Error> {
    // Lưu tin nhắn vào cơ sở dữ liệu và trả về tin nhắn đã lưu
    let saved_message = sqlx::query_as!(
        Message,
        r#"
        INSERT INTO messages (room_id, user_id, content, created_at)
        VALUES ($1, $2, $3, DEFAULT)
        RETURNING id, room_id, user_id, content, created_at
        "#,
        room_id,
        message.user_id,
        message.content
    )
    .fetch_one(pool)
    .await?;

    Ok(saved_message)
}

