use crate::entidades::{professor::Professor, projeto::Projeto};

#[derive(Debug)]
pub struct ProjetoComCoordenadores {
    pub projeto: Projeto,
    pub coordenador: Professor,
    pub vice_coordenador: Option<Professor>,
}
