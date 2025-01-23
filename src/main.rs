mod services;
mod models;
mod routes;
mod handlers;
mod ws;

use actix_web::{web, App, HttpServer};
use actix_web::http;
use sqlx::{postgres::PgPoolOptions};
use std::env;
use dotenv::dotenv;
use actix_cors::Cors;
use crate::routes::text_routes::config_routes;
use crate::routes::chat::chat_routes;
use crate::routes::user::user_routes;
use crate::routes::ws::ws_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Initialize logger
    env_logger::init();

    // Tạo kết nối tới PostgreSQL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Chạy server Actix
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000") // Cho phép origin này
                    .allowed_methods(vec!["GET", "POST", "DELETE"]) // Các phương thức được phép
                    .allowed_headers(vec![http::header::CONTENT_TYPE]) // Các header được phép
                    .supports_credentials() // Hỗ trợ cookie nếu cần
                )
            .service(ws_routes())
            .configure(config_routes)
            .configure(user_routes)
            .configure(chat_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
