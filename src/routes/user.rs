use actix_web::web;
use crate::handlers::user_handler::{register, login, get_users};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
        .route("/users", web::get().to(get_users));
}
