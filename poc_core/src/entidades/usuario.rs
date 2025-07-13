use chrono::{NaiveDateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{enums::cargo::Cargo, types::NullableU8};

#[derive(Debug, FromRow)]
pub struct Usuario {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
    pub curriculo_lattes: Option<String>,
    pub cargo: Cargo,
    pub ultimo_login_em: Option<NaiveDateTime>,
    pub atualizado_em: Option<NaiveDateTime>,
    pub criado_em: NaiveDateTime,
    pub registro_aluno: Option<String>,
    #[sqlx(try_from = "Option<i16>")]
    pub periodo: NullableU8,
}

#[derive(Debug, FromRow, Clone)]
pub struct UsuarioMinimo {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
    pub curriculo_lattes: Option<String>,
    pub ultimo_login_em: Option<NaiveDateTime>,
    pub atualizado_em: Option<NaiveDateTime>,
    pub criado_em: NaiveDateTime,
}

impl UsuarioMinimo {
    pub fn new(
        nome: String,
        curriculo_lattes: Option<String>,
        email: String,
        senha_hash: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            nome,
            senha_hash,
            curriculo_lattes,
            email,
            ultimo_login_em: None,
            criado_em: Utc::now().naive_utc(),
            atualizado_em: None,
        }
    }
}
