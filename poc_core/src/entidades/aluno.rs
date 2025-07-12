use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::entidades::usuario::UsuarioMinimo;

#[derive(Debug, FromRow)]
pub struct Aluno {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
    pub curriculo_lattes: Option<String>,
    // Não precisamos do cargo se sabemos que é um aluno...
    // pub cargo: Cargo,
    pub ultimo_login_em: Option<NaiveDateTime>,
    pub atualizado_em: Option<NaiveDateTime>,
    pub criado_em: NaiveDateTime,
    pub registro_aluno: String,
    pub periodo: i16,
}

#[derive(Debug, FromRow)]
pub struct UsuarioAluno {
    #[sqlx(flatten)]
    pub usuario: UsuarioMinimo,
    pub registro_aluno: String,
    pub periodo: i16,
}
