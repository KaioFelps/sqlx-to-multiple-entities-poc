use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{entidades::usuario::Usuario, enums::cargo::Cargo};

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

impl Professor {
    pub fn from_usuario(usuario: &Usuario) -> Self {
        Self {
            atualizado_em: usuario.atualizado_em,
            cargo: usuario.cargo.clone(),
            criado_em: usuario.criado_em,
            curriculo_lattes: usuario.curriculo_lattes.clone(),
            email: usuario.email.clone(),
            id: usuario.id,
            nome: usuario.nome.clone(),
            senha_hash: usuario.senha_hash.clone(),
            ultimo_login_em: usuario.ultimo_login_em,
        }
    }
}
