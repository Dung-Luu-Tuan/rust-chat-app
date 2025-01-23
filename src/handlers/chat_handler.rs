use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;
use crate::services::chat_service;
use crate::models::message::{NewMessage};

#[derive(Deserialize)]
pub struct NewRoom {
    pub user1_id: Uuid,
    pub user2_id: Uuid,
}

pub async fn create_room(
    pool: web::Data<sqlx::PgPool>,
    room: web::Json<NewRoom>, // Nhận thông tin về 2 user để tạo phòng
) -> impl Responder {
    match chat_service::get_or_create_room(
        pool.get_ref(),
        room.user1_id,
        room.user2_id,
    ).await {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create room"),
    }
}

pub async fn get_messages(
    pool: web::Data<sqlx::PgPool>,
    room_id: web::Path<Uuid>, // Lấy room_id từ URL
) -> impl Responder {
    // Lấy tất cả tin nhắn trong phòng
    let room_id = room_id.into_inner();
    match chat_service::fetch_messages(pool.get_ref(), &room_id).await {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch messages"),
    }
}

pub async fn save_message(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
    message: web::Json<NewMessage>,
) -> impl Responder {
    let room_id = path.into_inner();
    let user_id = message.user_id;     // Get user_id from JSON body
    let content = message.content.clone(); // Get content from JSON body

    let new_message = NewMessage {
        user_id,  // Pass user_id from the request body
        content,  // Pass content from the request body
    };

    match chat_service::save_message(pool.get_ref(), &new_message, room_id).await {
        Ok(saved_message) => HttpResponse::Ok().json(saved_message),
        Err(_) => HttpResponse::InternalServerError().body("Failed to save message"),
    }
}
