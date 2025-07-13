use sqlx::{Row, postgres::PgRow, prelude::FromRow};
use uuid::Uuid;

use crate::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;

#[derive(Debug)]
pub struct Vaga {
    pub id: Uuid,
    pub titulo_customizado: Option<String>,
    pub projeto: ProjetoComCoordenadores,
    pub detalhes: String,
    pub quantidade: u8,
    pub horas_p_semana: u8,
}

impl<'r> FromRow<'r, PgRow> for Vaga {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let qtd: i16 = row.try_get("quantidade")?;
        let hrs: i16 = row.try_get("horas_p_semana")?;

        Ok(Self {
            id: row.try_get("id")?,
            titulo_customizado: row.try_get("titulo_customizado")?,
            detalhes: row.try_get("detalhes")?,
            quantidade: qtd as u8,
            horas_p_semana: hrs as u8,
            projeto: ProjetoComCoordenadores::from_row(row)?,
        })
    }
}
