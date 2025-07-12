use poc_core::entidades::usuario::Usuario;
use poc_core::enums::cargo::Cargo;
use sqlx::PgConnection;

pub struct UsuariosRepo<'this> {
    pub db_conn: &'this mut PgConnection,
}

/*
    id                  UUID            NOT NULL    DEFAULT gen_random_uuid(),
    nome                VARCHAR(200)    NOT NULL,
    email               VARCHAR(200)    NOT NULL,
    senha_hash          VARCHAR(60)     NOT NULl,
    curriculo_lattes    VARCHAR(300),
    cargo               cargo_e         NOT NULL,
    ultimo_login_em     TIMESTAMP,
    atualizado_em       TIMESTAMP,
    criado_em           TIMESTAMP       NOT NULL    DEFAULT now(),

    --- específicos de entidades
    registro_aluno      VARCHAR(100),
    periodo             SMALLINT,
*/

impl UsuariosRepo<'_> {
    pub async fn salvar_usuario(&mut self, usuario: &Usuario) {
        sqlx::query!(
            "INSERT INTO usuario (id, nome, email, senha_hash, curriculo_lattes, \
            cargo, ultimo_login_em, atualizado_em, criado_em, periodo, registro_aluno) \
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            &usuario.id,
            &usuario.nome,
            &usuario.email,
            &usuario.senha_hash,
            usuario.curriculo_lattes.as_ref(),
            &usuario.cargo as &Cargo,
            usuario.ultimo_login_em.as_ref(),
            usuario.atualizado_em.as_ref(),
            &usuario.criado_em,
            usuario.periodo.as_ref(),
            usuario.registro_aluno.as_ref()
        )
        .execute(&mut *self.db_conn)
        .await
        .unwrap_or_else(|err| {
            panic!("Não foi possível inserir o usuario no banco de dados: {err}");
        });
    }
}
