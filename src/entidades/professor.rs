use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::enums::cargo::Cargo;

#[derive(Debug, FromRow)]
pub struct Professor {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
    pub curriculo_lattes: Option<String>,
    // O Cargo pode diferenciar um professor comum de um Administrador
    pub cargo: Cargo,
    pub ultimo_login_em: Option<NaiveDateTime>,
    pub atualizado_em: Option<NaiveDateTime>,
    pub criado_em: NaiveDateTime,
}
