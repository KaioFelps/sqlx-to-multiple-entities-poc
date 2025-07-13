use sqlx::query_as;
use sqlx_queries::projetos::ProjetosRepo;
use sqlx_queries::vagas::VagasRepo;
use utils::setup_and_get_db_conn;

use poc_core::entidades::aluno::{Aluno, UsuarioAluno};
use poc_core::entidades::professor::Professor;
use poc_core::entidades::usuario::Usuario;
use poc_core::enums::cargo::Cargo;
use uuid::uuid;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut db_conn = setup_and_get_db_conn().await;

    // region: --- Busca usuário como o modelo do usuário normal
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

    dbg!(&usuario);
    // endregion

    // region: --- Busca usuário como struct `Aluno`, um subset de `Usuario`
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

    dbg!(&usuario_como_aluno);
    // endregion

    // region: --- Busca usuário como struct `Professor`, um subset de `Usuario`
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

    dbg!(&usuario_como_professor);
    // endregion

    // region: --- Busca usuário como struct `Aluno`, que é **composto** por um `UsuarioMinimo`
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

    dbg!(&aluno_composto);
    // endregion

    // region: --- Busca agregação de projeto com coordenador e vice-coordenador
    let projeto = ProjetosRepo { db: &mut db_conn }
        .encontrar_projeto_com_coordenadores_por_id(&uuid!("23475197-8052-4711-b962-893d07a288aa"))
        .await;

    dbg!(projeto);
    // endregion

    // region: --- Busca uma vaga completa pelo seu ID
    let vaga = VagasRepo { db: &mut db_conn }
        .buscar_vaga_por_id(&uuid!("dc2748ae-ac44-489b-afc1-5eb946310a1e"))
        .await;
    dbg!(vaga);
    // endregion

    // region: --- Busca Vaga de um projeto com agregado e tudo mais
    let vagas = VagasRepo { db: &mut db_conn }.buscar_vagas().await;
    dbg!(vagas);
    // endregion

    Ok(())
}
