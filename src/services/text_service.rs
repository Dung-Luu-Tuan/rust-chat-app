use sqlx::PgPool;
use uuid::Uuid;
use crate::models::text::TextItem;

pub async fn get_texts_service(pool: &PgPool) -> Result<Vec<TextItem>, sqlx::Error> {
    let texts = sqlx::query_as!(TextItem, "SELECT * FROM texts")
        .fetch_all(pool)
        .await?;
    
    Ok(texts)
}

pub async fn add_text_service(pool: &PgPool, text: String) -> Result<Vec<TextItem>, sqlx::Error> {
    let new_id = Uuid::new_v4();
    
    let query_result = sqlx::query!(
        "INSERT INTO texts (id, text) VALUES ($1, $2)",
        new_id,
        text
    )
    .execute(pool)
    .await;

    match query_result {
        Ok(_) => get_texts_service(pool).await,
        Err(e) => Err(e),
    }
}


pub async fn delete_text_service(pool: &PgPool, id: Uuid) -> Result<Vec<TextItem>, sqlx::Error> {
    let query_result = sqlx::query!("DELETE FROM texts WHERE id = $1", id)
        .execute(pool)
        .await;

    match query_result {
        Ok(result) if result.rows_affected() > 0 => get_texts_service(pool).await, // Delete successfully
        Ok(_) => Err(sqlx::Error::RowNotFound),
        Err(e) => Err(e),
    }
}
