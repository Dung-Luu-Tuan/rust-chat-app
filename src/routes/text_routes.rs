use actix_web::web;
use crate::handlers::text_handler::{get_texts, add_text, delete_text};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/texts", web::get().to(get_texts))
        .route("/texts", web::post().to(add_text))
        .route("/texts/{id}", web::delete().to(delete_text));
}
