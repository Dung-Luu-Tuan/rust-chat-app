use actix_web::{web, HttpResponse, Responder};
use crate::services::text_service::{add_text_service, get_texts_service, delete_text_service};
use crate::models::text::NewText;
use uuid::Uuid;

// Handler for GET request to fetch texts
pub async fn get_texts(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match get_texts_service(pool.as_ref()).await {
        Ok(texts) => HttpResponse::Ok().json(texts), // Return the texts data if successful
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch texts"), // Return error if failed
    }
}

// Handler for POST request to add a new text
pub async fn add_text(pool: web::Data<sqlx::PgPool>, item: web::Json<NewText>) -> impl Responder {
    let text = item.text.clone(); // Clone the text field from the incoming request

    match add_text_service(pool.as_ref(), text).await {
        Ok(texts) => HttpResponse::Ok().json(texts), // Return the newly added text if successful
        Err(_) => HttpResponse::InternalServerError().body("Failed to add text"), // Return error if failed
    }
}

// Handler for DELETE request to remove a text by its ID
pub async fn delete_text(pool: web::Data<sqlx::PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner(); // Extract the ID from the URL path

    match delete_text_service(pool.as_ref(), id).await {
        Ok(texts) => HttpResponse::Ok().json(texts), // Return success response if deletion was successful
        Err(_) => HttpResponse::NotFound().body("Text not found"), // Return 404 if text is not found
    }
}
