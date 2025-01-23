use actix_web::{web, Scope};
use crate::ws::handler::ws_index;

pub fn ws_routes() -> Scope {
    web::scope("/ws")
        .route("/{room_id}", web::get().to(ws_index))
}
