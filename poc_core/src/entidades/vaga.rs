use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;

#[derive(Debug, FromRow)]
pub struct Vaga {
    pub id: Uuid,
    pub titulo_customizado: Option<String>,
    #[sqlx(flatten)]
    pub projeto: ProjetoComCoordenadores,
    pub detalhes: String,
    #[sqlx(try_from = "i16")]
    pub quantidade: u8,
    #[sqlx(try_from = "i16")]
    pub horas_p_semana: u8,
}
