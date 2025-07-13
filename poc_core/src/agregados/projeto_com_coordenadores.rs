use sqlx::{Row, postgres::PgRow, prelude::FromRow};
use uuid::Uuid;

use crate::entidades::{professor::Professor, projeto::Projeto};

#[derive(Debug, Clone)]
pub struct ProjetoComCoordenadores {
    pub projeto: Projeto,

    pub coordenador: Professor,

    pub vice_coordenador: Option<Professor>,
}

impl<'r> FromRow<'r, PgRow> for ProjetoComCoordenadores {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let vice_id: Option<Uuid> = row.try_get("vice_id")?;

        Ok(Self {
            projeto: Projeto {
                id: row.try_get("p_id")?,
                title: row.try_get("p_title")?,
                content: row.try_get("p_content")?,
                created_at: row.try_get("p_created_at")?,
            },
            coordenador: Professor {
                id: row.try_get("coorde_id")?,
                nome: row.try_get("coorde_nome")?,
                email: row.try_get("coorde_email")?,
                senha_hash: row.try_get("coorde_senha_hash")?,
                curriculo_lattes: row.try_get("coorde_curriculo_lattes")?,
                cargo: row.try_get("coorde_cargo")?,
                ultimo_login_em: row.try_get("coorde_ultimo_login_em")?,
                atualizado_em: row.try_get("coorde_atualizado_em")?,
                criado_em: row.try_get("coorde_criado_em")?,
            },
            vice_coordenador: match vice_id {
                None => None,
                Some(id) => Some(Professor {
                    id,
                    nome: row.try_get("vice_nome")?,
                    email: row.try_get("vice_email")?,
                    senha_hash: row.try_get("vice_senha_hash")?,
                    curriculo_lattes: row.try_get("vice_curriculo_lattes")?,
                    cargo: row.try_get("vice_cargo")?,
                    ultimo_login_em: row.try_get("vice_ultimo_login_em")?,
                    atualizado_em: row.try_get("vice_atualizado_em")?,
                    criado_em: row.try_get("vice_criado_em")?,
                }),
            },
        })
    }
}
