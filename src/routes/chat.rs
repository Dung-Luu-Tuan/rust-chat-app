use actix_web::{web};
use crate::handlers::chat_handler::{create_room, save_message, get_messages};

pub fn chat_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/room")
            .route(web::post().to(create_room)) // Tạo hoặc lấy phòng
    )
    .service(
        web::resource("/messages/{room_id}")
            .route(web::get().to(get_messages)) // Lấy tất cả tin nhắn trong phòng
            .route(web::post().to(save_message)) // Lưu tin nhắn mới
    );
}
