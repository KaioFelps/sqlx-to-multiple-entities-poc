use chrono::Utc;
use poc_core::{entidades::usuario::Usuario, enums::cargo::Cargo};
use sqlx::PgConnection;
use uuid::Uuid;

pub async fn inserir_usuarios(db: &mut PgConnection) {
    let aluno = Usuario {
        atualizado_em: None,
        cargo: Cargo::Aluno,
        criado_em: Utc::now().naive_utc(),
        curriculo_lattes: None,
        email: "ricardo@gmail.com".to_string(),
        id: Uuid::new_v4(),
        nome: "Ricardo".into(),
        periodo: Some(2),
        registro_aluno: Some("a2600000".into()),
        senha_hash: "123".into(),
        ultimo_login_em: None,
    };

    let professor = Usuario {
        atualizado_em: None,
        cargo: Cargo::Professor,
        criado_em: Utc::now().naive_utc(),
        curriculo_lattes: None,
        email: "mario@gmail.com".to_string(),
        id: Uuid::new_v4(),
        nome: "Mário".into(),
        periodo: None,
        registro_aluno: None,
        senha_hash: "123".into(),
        ultimo_login_em: None,
    };

    let mut repo = sqlx_queries::usuarios::UsuariosRepo { db_conn: db };
    repo.salvar_usuario(&aluno).await;
    repo.salvar_usuario(&professor).await;

    printar_credenciais(&aluno);
    printar_credenciais(&professor);
}

fn printar_credenciais(usuario: &Usuario) {
    println!(
        "{}: {{ id: {}, cargo: {:#?}, ra: {:?}, email: {} }}",
        usuario.nome, usuario.id, usuario.cargo, usuario.registro_aluno, usuario.email
    );
}
