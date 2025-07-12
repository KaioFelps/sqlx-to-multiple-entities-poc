use sqlx::prelude::FromRow;

use crate::entidades::{professor::Professor, projeto::Projeto};

#[derive(Debug, FromRow)]
pub struct ProjetoComCoordenadores {
    #[sqlx(flatten)]
    pub projeto: Projeto,

    #[sqlx(flatten)]
    pub coordenador: Professor,

    #[sqlx(flatten)]
    pub vice_coordenador: Option<Professor>,
}
