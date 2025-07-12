use chrono::Utc;
use poc_core::{
    agregados::projeto_com_coordenadores::ProjetoComCoordenadores,
    entidades::{professor::Professor, projeto::Projeto, usuario::Usuario},
};
use sqlx::PgConnection;
use sqlx_queries::projetos::ProjetosRepo;
use uuid::uuid;

pub async fn inserir_projetos(db: &mut PgConnection, coordenador: &Usuario, vice_coord: &Usuario) {
    let projeto_sem_vice = ProjetoComCoordenadores {
        projeto: Projeto {
            id: uuid!("f0169deb-5653-4274-97cd-53c4c2f43c57"),
            title: "Titulo do Projeto".into(),
            content: "<h1>Titulo do projeto</h1><p>Descrição do projeto</p>".into(),
            created_at: Utc::now().naive_utc(),
        },
        coordenador: Professor::from_usuario(coordenador),
        vice_coordenador: None,
    };

    let projeto_com_vice = ProjetoComCoordenadores {
        projeto: Projeto {
            id: uuid!("23475197-8052-4711-b962-893d07a288aa"),
            title: "Algebra na Otimização de Algoritmos".into(),
            content: "<h1>Titulo do projeto</h1><p>Descrição do projeto</p>".into(),
            created_at: Utc::now().naive_utc(),
        },
        coordenador: Professor::from_usuario(vice_coord),
        vice_coordenador: Some(Professor::from_usuario(coordenador)),
    };

    let mut repo = ProjetosRepo { db };
    repo.inserir_projeto(&projeto_sem_vice).await;
    repo.inserir_projeto(&projeto_com_vice).await;

    printar_projeto(&projeto_sem_vice);
    printar_projeto(&projeto_com_vice);
}

fn printar_projeto(projeto: &ProjetoComCoordenadores) {
    println!(
        "{}: {{ coordenador: {}, vice: {} }}",
        projeto.projeto.title,
        projeto.coordenador.nome,
        projeto
            .vice_coordenador
            .as_ref()
            .map(|c| c.nome.as_str())
            .unwrap_or("Não tem")
    )
}
