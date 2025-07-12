use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;

#[derive(Debug, FromRow)]
pub struct Vaga {
    pub id: Uuid,
    pub titulo_customizado: Option<String>,
    pub projeto: ProjetoComCoordenadores,
    pub detalhes: String,
    pub quantidade: u8,
    pub horas_p_semana: u8,
}
