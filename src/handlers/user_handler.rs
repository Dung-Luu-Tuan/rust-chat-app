use actix_web::{web, HttpResponse, Responder};
use crate::services::user_service::{register_user, login_user, fetch_users};
use crate::models::user::{RegisterUser, LoginUser};

pub async fn register(pool: web::Data<sqlx::PgPool>, item: web::Json<RegisterUser>) -> impl Responder {
    match register_user(pool.as_ref(), &item.username, &item.email, &item.password).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Failed to register user"),
    }
}

pub async fn login(pool: web::Data<sqlx::PgPool>, item: web::Json<LoginUser>) -> impl Responder {
    match login_user(pool.as_ref(), &item.username, &item.password).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

pub async fn get_users(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match fetch_users(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch users"),
    }
}