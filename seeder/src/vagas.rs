use poc_core::{
    agregados::projeto_com_coordenadores::ProjetoComCoordenadores, entidades::vaga::Vaga,
};
use sqlx::PgConnection;
use sqlx_queries::vagas::VagasRepo;
use uuid::uuid;

pub async fn inserir_vagas(
    db: &mut PgConnection,
    projeto_com_vice: &ProjetoComCoordenadores,
    projeto_sem_vice: &ProjetoComCoordenadores,
) -> (Vaga, Vaga) {
    let vaga_1 = Vaga {
        id: uuid!("dc2748ae-ac44-489b-afc1-5eb946310a1e"),
        detalhes: "Se inscreva para o prejeto tal!".into(),
        horas_p_semana: 20,
        quantidade: 3,
        titulo_customizado: None,
        projeto: projeto_com_vice.clone(),
    };

    let vaga_2 = Vaga {
        id: uuid!("c6f86834-eb2f-4450-85f4-7c753df54694"),
        detalhes: "Detalhes do outro projeto".into(),
        horas_p_semana: 25,
        quantidade: 2,
        projeto: projeto_sem_vice.clone(),
        titulo_customizado: Some("Título customizado".into()),
    };

    let mut repo = VagasRepo { db };
    repo.salvar_vaga(&vaga_1).await;
    repo.salvar_vaga(&vaga_2).await;

    printar_vaga(&vaga_1);
    printar_vaga(&vaga_2);

    (vaga_1, vaga_2)
}

fn printar_vaga(vaga: &Vaga) {
    println!(
        r#"Vaga "{}": {{ projeto: {}, coord: {}, vice: {} }}"#,
        vaga.titulo_customizado
            .as_ref()
            .unwrap_or(&vaga.projeto.projeto.title),
        vaga.projeto.projeto.title,
        vaga.projeto.coordenador.nome,
        vaga.projeto
            .vice_coordenador
            .as_ref()
            .map(|coord| coord.nome.as_str())
            .unwrap_or("Não há vice."),
    );
}
