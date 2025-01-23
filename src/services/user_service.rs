use crate::models::user::{User, RespondUser};
use sqlx::PgPool;
use bcrypt::{verify, hash, DEFAULT_COST};

pub async fn register_user(pool: &PgPool, username: &str, email: &str, password: &str) -> Result<RespondUser, sqlx::Error> {
    let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");

    let new_user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, password_hash, created_at
        "#,
        username,
        email,
        hashed_password
    )
    .fetch_one(pool)
    .await?;

    let response_user = RespondUser {
        id: new_user.id,
        username: new_user.username,
        email: new_user.email,
        created_at: new_user.created_at,
    };
    Ok(response_user)
}

pub async fn login_user(pool: &PgPool, username: &str, password: &str) -> Result<RespondUser, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, created_at
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_one(pool)
    .await?;

    if verify(password, &user.password_hash).unwrap_or(false) {
        let response_user = RespondUser {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        };
        Ok(response_user)
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}

pub async fn fetch_users(pool: &PgPool) -> Result<Vec<RespondUser>, sqlx::Error> {
    // Truy vấn tất cả người dùng
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, created_at
        FROM users
        "#,
    )
    .fetch_all(pool)
    .await?;

    // Chuyển đổi từ User sang RespondUser
    let response_users: Vec<RespondUser> = users
        .into_iter()
        .map(|user| RespondUser {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        })
        .collect();

    Ok(response_users)
}
