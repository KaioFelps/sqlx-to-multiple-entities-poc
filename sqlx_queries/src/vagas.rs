use core::panic;

use poc_core::entidades::vaga::Vaga;
use sqlx::PgConnection;
use uuid::Uuid;

pub struct VagasRepo<'this> {
    pub db: &'this mut PgConnection,
}

impl VagasRepo<'_> {
    pub async fn salvar_vaga(&mut self, vaga: &Vaga) {
        sqlx::query!(
            r#"INSERT INTO vaga (
                id,
                id_projeto,
                titulo_customizado,
                detalhes,
                quantidade,
                horas_p_semana
            ) VALUES ($1, $2, $3, $4, $5, $6)"#,
            &vaga.id,
            &vaga.projeto.projeto.id,
            vaga.titulo_customizado.as_ref(),
            &vaga.detalhes,
            vaga.quantidade as i16,
            vaga.horas_p_semana as i16
        )
        .execute(&mut *self.db)
        .await
        .unwrap_or_else(|err| panic!("{err}"));
    }

    pub async fn buscar_vaga_por_id(&mut self, id: &Uuid) -> Option<Vaga> {
        sqlx::query_as(
            r#"SELECT v.*, p.*, coord.*, vice.*
            FROM vaga v
            INNER JOIN projeto p ON p.id = v.id_projeto
            INNER JOIN projeto_coordenadores rel_coord ON rel_coord.id_projeto = p.id AND rel_coord.tipo_coord = 'coord'
            INNER JOIN usuario coord ON coord.id = rel_coord.id_coord
            LEFT JOIN projeto_coordenadores rel_vice ON rel_vice.id_projeto = p.id AND rel_vice.tipo_coord = 'vice'
            LEFT JOIN usuario vice ON vice.id = rel_vice.id_coord
            WHERE v.id = $1"#,
        ).bind(id).fetch_optional(&mut *self.db).await.unwrap_or_else(|err| panic!("{err}"))
    }
}

/*  vaga (
    id                  UUID            NOT NULL    DEFAULT gen_random_uuid(),
    id_projeto          UUID            NOT NULL,
    titulo_customizado  VARCHAR(200),
    detalhes            TEXT            NOT NULL,
    quantidade          SMALLINT        NOT NULL,
    horas_p_semana      SMALLINT        NOT NULL,
)
 */
