mod projetos;
mod usuarios;

#[tokio::main]
async fn main() {
    let mut db_conn = utils::setup_and_get_db_conn().await;

    #[cfg(feature = "resetar")]
    sqlx::query!("DELETE FROM usuario")
        .execute(&mut db_conn)
        .await
        .expect("Não foi possível resetar o banco de dados.");

    let (_aluno, mario, sara) = usuarios::inserir_usuarios(&mut db_conn).await;

    projetos::inserir_projetos(&mut db_conn, &mario, &sara).await;
}
