use sqlx::query_as;
use utils::setup_and_get_db_conn;

use poc_core::entidades::aluno::{Aluno, UsuarioAluno};
use poc_core::entidades::professor::Professor;
use poc_core::entidades::usuario::Usuario;
use poc_core::enums::cargo::Cargo;

pub mod sqlx_queries;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut db_conn = setup_and_get_db_conn().await;

    let usuario = query_as!(
        Usuario,
        r#"SELECT 
            id, 
            nome, 
            email, 
            senha_hash, 
            curriculo_lattes, 
            cargo AS "cargo: Cargo",
            ultimo_login_em, 
            atualizado_em, 
            criado_em,
            registro_aluno, 
            periodo
        FROM usuario LIMIT 1"#
    )
    .fetch_one(&mut db_conn)
    .await
    .unwrap();

    println!("{usuario:#?}");

    let usuario_como_aluno = query_as!(
        Aluno,
        r#"SELECT
            id,
            nome,
            email,
            senha_hash,
            curriculo_lattes,
            ultimo_login_em,
            atualizado_em,
            criado_em,
            registro_aluno AS "registro_aluno!: String",
            periodo AS "periodo!: i16"
        FROM usuario
        WHERE
            usuario.cargo = 'aluno'
            AND usuario.registro_aluno IS NOT NULL
            AND usuario.periodo IS NOT NULL
        LIMIT 1
        "#,
    )
    .fetch_one(&mut db_conn)
    .await
    .unwrap();

    println!("{usuario_como_aluno:#?}");

    let usuario_como_professor = query_as!(
        Professor,
        r#"SELECT
            id,
            nome,
            email,
            senha_hash,
            cargo AS "cargo: Cargo",
            curriculo_lattes,
            ultimo_login_em,
            atualizado_em,
            criado_em
        FROM usuario
        WHERE
            usuario.cargo in ('professor', 'administrador')
        LIMIT 1
        "#
    )
    .fetch_one(&mut db_conn)
    .await
    .unwrap();

    println!("{usuario_como_professor:#?}");

    // Busca por entidades compostas funciona somente com a função `query_as`
    // (observe que não é o macro `query_as!`).
    // desde que a esturtura aninhada seja decorada com o atributo `sqlx(flatten)`.
    // nesse caso também não há como utilizar o `!` para garantir valores não-nulos // nem como parsear diretamente para um tipo rust.  // Apesar disso, esses aparatos não são necessários se a query SQL estiver correta:
    // já garantimos que `registro_aluno` e `periodo` não serão nulos na cláusula `WHERE`.
    let aluno_composto: UsuarioAluno = query_as(
        r#"SELECT
            id,
            nome,
            email,
            senha_hash,
            curriculo_lattes,
            ultimo_login_em,
            atualizado_em,
            criado_em,
            registro_aluno,
            periodo
        FROM usuario
        WHERE
            usuario.cargo = 'aluno'
            AND usuario.registro_aluno IS NOT NULL
            AND usuario.periodo IS NOT NULL
        LIMIT 1
        "#,
    )
    .fetch_one(&mut db_conn)
    .await
    .unwrap();

    println!("{aluno_composto:#?}");

    Ok(())
}
