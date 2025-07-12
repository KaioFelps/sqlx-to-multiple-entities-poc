use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{
    entidades::{professor::Professor, projeto::Projeto},
    enums::cargo::Cargo,
};

#[derive(Debug)]
pub struct ProjetoComCoordenadores {
    pub projeto: Projeto,

    pub coordenador: Professor,

    pub vice_coordenador: Option<Professor>,
}

#[derive(FromRow, Debug)]
pub struct ProjetoComCoordenadoresTupla(
    pub Uuid,
    pub String,
    pub String,
    pub NaiveDateTime,
    // coord
    pub Uuid,
    pub String,
    pub String,
    pub String,
    pub Option<String>,
    pub Cargo,
    pub Option<NaiveDateTime>,
    pub Option<NaiveDateTime>,
    pub NaiveDateTime,
    // vice
    pub Option<Uuid>,
    pub Option<String>,
    pub Option<String>,
    pub Option<String>,
    pub Option<String>,
    pub Option<Cargo>,
    pub Option<NaiveDateTime>,
    pub Option<NaiveDateTime>,
    pub Option<NaiveDateTime>,
);

impl From<ProjetoComCoordenadoresTupla> for ProjetoComCoordenadores {
    fn from(value: ProjetoComCoordenadoresTupla) -> Self {
        let ProjetoComCoordenadoresTupla(
            p_id,
            p_title,
            p_content,
            p_created_at,
            coorde_id,
            coorde_nome,
            coorde_email,
            coorde_senha_hash,
            coorde_curriculo_lattes,
            coorde_cargo,
            coorde_ultimo_login_em,
            coorde_atualizado_em,
            coorde_criado_em,
            vice_id,
            vice_nome,
            vice_email,
            vice_senha_hash,
            vice_curriculo_lattes,
            vice_cargo,
            vice_ultimo_login_em,
            vice_atualizado_em,
            vice_criado_em,
        ) = value;

        Self {
            coordenador: Professor {
                id: coorde_id,
                nome: coorde_nome,
                email: coorde_email,
                senha_hash: coorde_senha_hash,
                cargo: coorde_cargo,
                curriculo_lattes: coorde_curriculo_lattes,
                atualizado_em: coorde_atualizado_em,
                ultimo_login_em: coorde_ultimo_login_em,
                criado_em: coorde_criado_em,
            },
            vice_coordenador: vice_id.map(|id| Professor {
                id,
                nome: vice_nome.unwrap(),
                cargo: vice_cargo.unwrap(),
                curriculo_lattes: vice_curriculo_lattes,
                email: vice_email.unwrap(),
                senha_hash: vice_senha_hash.unwrap(),
                ultimo_login_em: vice_ultimo_login_em,
                atualizado_em: vice_atualizado_em,
                criado_em: vice_criado_em.unwrap(),
            }),
            projeto: Projeto {
                id: p_id,
                content: p_content,
                created_at: p_created_at,
                title: p_title,
            },
        }
    }
}
