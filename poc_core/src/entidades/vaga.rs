use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;

#[derive(Debug, FromRow)]
pub struct Vaga {
    id: Uuid,
    titulo_customizado: Option<String>,
    projeto: ProjetoComCoordenadores,
    detalhes: String,
    quantidade: u8,
    horas_p_semana: u8,
}
