use poc_core::agregados::projeto_com_coordenadores::{
    ProjetoComCoordenadores, ProjetoComCoordenadoresTupla,
};
use sqlx::PgConnection;
use uuid::Uuid;

pub struct ProjetosRepo<'this> {
    pub db: &'this mut PgConnection,
}

impl ProjetosRepo<'_> {
    pub async fn inserir_projeto(&mut self, projeto: &ProjetoComCoordenadores) {
        sqlx::query!(
            "INSERT INTO projeto (id, title, content, created_at) VALUES ($1, $2, $3, $4)",
            &projeto.projeto.id,
            &projeto.projeto.title,
            &projeto.projeto.content,
            projeto.projeto.created_at
        )
        .execute(&mut *self.db)
        .await
        .unwrap_or_else(|err| panic!("{err}"));

        sqlx::query!(
            "INSERT INTO projeto_coordenadores (id_projeto, id_coord, tipo_coord) VALUES ($1, $2, 'coord')",
            projeto.projeto.id,
            projeto.coordenador.id
        ).execute(&mut *self.db).await.unwrap_or_else(|err| panic!("{err}"));

        if let Some(vice) = &projeto.vice_coordenador {
            sqlx::query!(
                "INSERT INTO projeto_coordenadores (id_projeto, id_coord, tipo_coord) VALUES ($1, $2, 'vice')",
                &projeto.projeto.id,
                &vice.id
            )
            .execute(&mut *self.db)
            .await
            .expect("Não foi possível adicionar o coordenador.");
        }
    }

    pub async fn encontrar_projeto_com_coordenadores_por_id(
        &mut self,
        id_projeto: &Uuid,
    ) -> Option<ProjetoComCoordenadores> {
        let resultado : Option<ProjetoComCoordenadoresTupla> = sqlx::query_as(
            r#"SELECT
                p.id,
                p.title,
                p.content,
                p.created_at,
                coorde.id,
                coorde.nome,
                coorde.email,
                coorde.senha_hash,
                coorde.curriculo_lattes,
                coorde.cargo,
                coorde.ultimo_login_em,
                coorde.atualizado_em,
                coorde.criado_em,
                vice.id,
                vice.nome,
                vice.email,
                vice.senha_hash,
                vice.curriculo_lattes,
                vice.cargo,
                vice.ultimo_login_em,
                vice.atualizado_em,
                vice.criado_em
            FROM projeto p
            INNER JOIN projeto_coordenadores rel_coord ON rel_coord.id_projeto = p.id AND rel_coord.tipo_coord = 'coord'
            INNER JOIN usuario coorde ON coorde.id = rel_coord.id_coord
            LEFT JOIN projeto_coordenadores rel_vice ON rel_vice.id_projeto = p.id AND rel_vice.tipo_coord = 'vice'
            LEFT JOIN usuario vice ON vice.id = rel_vice.id_coord
            WHERE p.id = $1"#,
        )
            .bind(id_projeto)
            .fetch_optional(&mut *self.db)
            .await
            .unwrap_or_else(|err| panic!("{err}"));

        resultado.map(ProjetoComCoordenadores::from)
    }
}

/*
projeto (
    id                  UUID            NOT NULL    DEFAULT gen_random_uuid(),
    title               VARCHAR(200)    NOT NULL,
    content             TEXT            NOT NULL,
    created_at          TIMESTAMP       NOT NULL    DEFAULT now(),

    CONSTRAINT projeto_pk PRIMARY KEY (id)
);

projeto_coordenadores (
    id                  UUID            NOT NULL    DEFAULT gen_random_uuid(),
    id_projeto          UUID            NOT NULL,
    id_coord            UUID            NOT NULL,
    tipo_coord          tipo_coord      NOT NULL,
)
*/
