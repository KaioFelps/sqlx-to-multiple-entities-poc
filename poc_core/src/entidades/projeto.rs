use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Clone)]
pub struct Projeto {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}
