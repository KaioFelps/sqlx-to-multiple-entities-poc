mod usuarios;

#[tokio::main]
async fn main() {
    let mut db_conn = utils::setup_and_get_db_conn().await;

    #[cfg(feature = "resetar")]
    sqlx::query!("DELETE FROM usuario")
        .execute(&mut db_conn)
        .await
        .expect("Não foi possível resetar o banco de dados.");

    usuarios::inserir_usuarios(&mut db_conn).await;
}
