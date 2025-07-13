mod projetos;
mod usuarios;
mod vagas;

#[tokio::main]
async fn main() {
    let mut db_conn = utils::setup_and_get_db_conn().await;

    #[cfg(feature = "resetar")]
    {
        sqlx::query!("DELETE FROM vaga")
            .execute(&mut db_conn)
            .await
            .expect("Não foi possível resetar o banco de dados.");

        sqlx::query!("DELETE FROM projeto_coordenadores")
            .execute(&mut db_conn)
            .await
            .expect("Não foi possível resetar o banco de dados.");

        sqlx::query!("DELETE FROM projeto")
            .execute(&mut db_conn)
            .await
            .expect("Não foi possível resetar o banco de dados.");

        sqlx::query!("DELETE FROM usuario")
            .execute(&mut db_conn)
            .await
            .expect("Não foi possível resetar o banco de dados.");
    }

    let (_aluno, mario, sara) = usuarios::inserir_usuarios(&mut db_conn).await;

    let (projeto_com_vice, projeto_sem_vice) =
        projetos::inserir_projetos(&mut db_conn, &mario, &sara).await;

    let (_vaga1, _vaga2) =
        vagas::inserir_vagas(&mut db_conn, &projeto_com_vice, &projeto_sem_vice).await;
}
